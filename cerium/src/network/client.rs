use bytes::BytesMut;
use parking_lot::{Mutex, RwLock};
use std::{
    io::Cursor,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::mpsc;
use tokio::{
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    Server,
    auth::KeyStore,
    entity::Player,
    network::{reader::StreamReader, writer::StreamWriter},
    protocol::{
        ProtocolState,
        encode::{EncodeError, packet_id},
        packet::{DisconnectPacket, Packet},
    },
    text::Component,
};
use crate::{
    auth::GameProfile,
    protocol::{encode::PacketWrite as _, packet::ServerPacket},
};

pub struct Connection {
    addr: SocketAddr,
    sreader: tokio::sync::Mutex<StreamReader<OwnedReadHalf>>,
    swriter: tokio::sync::Mutex<StreamWriter<OwnedWriteHalf>>,
    packet_tx: Sender<BytesMut>,
    state: RwLock<ProtocolState>,
    pub(crate) game_profile: Mutex<Option<GameProfile>>,
    pub(crate) key_store: Arc<KeyStore>,
    pub(crate) verify_token: Mutex<[u8; 4]>,
    pub(crate) player: Mutex<Option<Arc<Player>>>,
    closed: AtomicBool,
    server: Arc<Server>,
}

impl Connection {
    pub fn new(
        addr: SocketAddr,
        stream: TcpStream,
        server: Arc<Server>,
    ) -> (Arc<Self>, Receiver<BytesMut>) {
        let (rstream, wstream) = stream.into_split();
        let (tx, rx) = mpsc::channel(128);

        let connection = Arc::new(Self {
            addr,
            sreader: tokio::sync::Mutex::new(StreamReader::new(rstream)),
            swriter: tokio::sync::Mutex::new(StreamWriter::new(wstream)),
            packet_tx: tx,
            state: RwLock::new(ProtocolState::Handshake),
            game_profile: Mutex::new(None),
            key_store: server.key_store(),
            verify_token: Mutex::new([0; 4]),
            player: Mutex::new(None),
            closed: AtomicBool::new(false),
            server,
        });

        (connection, rx)
    }

    pub async fn accept(addr: SocketAddr, stream: TcpStream, server: Arc<Server>) {
        let (conn, mut rx) = Connection::new(addr, stream, server.clone());

        let rtask = server.handle().spawn({
            let conn = conn.clone();
            async move {
                conn.read_loop().await;
            }
        });
        let wtask = server.handle().spawn({
            let conn = conn.clone();
            async move {
                conn.write_loop(&mut rx).await;
            }
        });

        tokio::try_join!(rtask, wtask).unwrap();

        let player = conn.player.lock();
        if player.is_some() {
            let player = player.clone().unwrap();
            player.despawn();
        }
        server.players.lock().retain(|p| p.addr() != addr);
    }

    pub async fn set_compression(&self, threshold: i32) {
        self.swriter.lock().await.set_compression(threshold);
        self.sreader.lock().await.set_compression(threshold);
    }

    pub async fn set_encryption(&self, shared_secret: &[u8]) {
        self.sreader.lock().await.set_encryption(shared_secret);
        self.swriter.lock().await.set_encryption(shared_secret);
    }

    pub async fn read_loop(self: Arc<Self>) {
        let this = self.clone();
        while !this.closed() {
            let this = this.clone();
            let packet = {
                let mut reader = this.sreader.lock().await;
                match reader.read_packet().await {
                    Ok(v) => v,
                    Err(_) => break,
                }
            }; // reader (MutexGuard) dropped here

            if let Err(e) = this
                .handle_packet(packet.id(), &mut Cursor::new(packet.data()))
                .await
            {
                log::error!("Failed to handle packet: {}", e);
                break;
            }
        }
    }

    pub async fn write_loop(self: Arc<Self>, rx: &mut Receiver<BytesMut>) {
        while !self.closed() {
            let Some(data) = rx.recv().await else {
                self.close();
                break;
            };

            self.write_packet(data).await;
        }
    }

    pub fn set_state(&self, state: ProtocolState) {
        *self.state.write() = state;
    }

    pub fn state(&self) -> ProtocolState {
        *self.state.read()
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn send_packet<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        let data = match self.encode_packet(packet) {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };

        // Enqueue packet
        if let Err(_) = self.packet_tx.try_send(data) {
            log::warn!("Failed to enqueue packet. ({})", std::any::type_name::<P>());
            self.close();
        }
    }

    pub async fn send_packet_now<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        let data = match self.encode_packet(packet) {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };

        // Write the packet immediately
        self.write_packet(data).await;
    }

    fn encode_packet<P>(&self, packet: P) -> Result<BytesMut, EncodeError>
    where
        P: Packet + ServerPacket + 'static,
    {
        let state = self.state.try_read().unwrap();

        let Some(packet_id) = packet_id::<P>(&state) else {
            log::error!(
                "Failed to resolve ID for packet. ({})",
                std::any::type_name::<P>()
            );
            self.close();
            return Err(EncodeError::Encode("".to_string()));
        };

        let mut data = BytesMut::new();
        data.write_varint(packet_id)?;
        P::encode(&mut data, &packet)?;
        Ok(data)
    }

    async fn write_packet(&self, data: BytesMut) {
        let mut swriter = self.swriter.lock().await;

        if let Err(err) = swriter.write_packet(&data).await {
            log::error!("Failed to send packet: {}", err);
            self.close();
            return;
        }
    }

    pub fn kick(&self, reason: Component) {
        match *self.state.try_read().unwrap() {
            ProtocolState::Login => {} // LoginDisconnectPacket { reason: todo!() },
            _ => self.send_packet(DisconnectPacket { reason }),
        }
        self.close();
    }

    pub fn server(&self) -> &Arc<Server> {
        &self.server
    }

    pub fn closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
    }
}
