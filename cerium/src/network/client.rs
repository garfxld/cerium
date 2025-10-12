use bytes::BytesMut;
use std::{
    io::Cursor,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::net::{
    TcpStream,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
};
use tokio::sync::Mutex;

use crate::{
    Server,
    auth::KeyStore,
    entity::Player,
    network::{reader::StreamReader, writer::StreamWriter},
    protocol::{
        ProtocolState,
        packet::{Packet, server::play},
    },
    text::Component,
};
use crate::{
    auth::GameProfile,
    protocol::{encode::PacketWrite as _, packet::ServerPacket},
};

pub struct ClientConnection {
    addr: SocketAddr,
    sreader: Mutex<StreamReader<OwnedReadHalf>>,
    swriter: Mutex<StreamWriter<OwnedWriteHalf>>,

    pub state: Mutex<ProtocolState>,
    pub game_profile: Mutex<Option<GameProfile>>,
    pub key_store: Arc<KeyStore>,
    pub verify_token: Mutex<[u8; 4]>,
    pub player: Mutex<Option<Arc<Player>>>,
    pub closed: AtomicBool,
    pub server: Arc<Server>,
}

impl ClientConnection {
    pub fn new(addr: SocketAddr, stream: TcpStream, server: Arc<Server>) -> Self {
        let (rstream, wstream) = stream.into_split();

        Self {
            addr,
            sreader: Mutex::new(StreamReader::new(rstream)),
            swriter: Mutex::new(StreamWriter::new(wstream)),

            state: Mutex::new(ProtocolState::Handshake),
            game_profile: Mutex::new(None),
            key_store: server.key_store(),
            verify_token: Mutex::new([0; 4]),
            player: Mutex::new(None),
            closed: AtomicBool::new(false),
            server,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub async fn read_loop(self: Arc<Self>) {
        let this = self.clone();
        while !this.closed() {
            let this = this.clone();

            let packet = {
                let mut decoder = self.sreader.lock().await;
                match decoder.read_packet().await {
                    Ok(v) => v,
                    Err(_) => break,
                }
            };
            let mut data = Cursor::new(packet.data());

            if let Err(e) = this.handle_packet(packet.id(), &mut data).await {
                log::error!("error: {}", e);
                break;
            };
        }
    }

    pub async fn set_encryption(&self, shared_secret: &[u8]) {
        log::debug!("set_encryption {:?}", shared_secret);
        self.sreader.lock().await.set_encryption(shared_secret);
        self.swriter.lock().await.set_encryption(shared_secret);
    }

    pub async fn set_compression(&self, threshold: i32) {
        log::debug!("set_compression {}", threshold);
        self.sreader.lock().await.set_compression(threshold);
        self.swriter.lock().await.set_compression(threshold);
    }

    async fn state(&self) -> ProtocolState {
        *self.state.lock().await
    }

    pub async fn send_packet<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        let mut data = BytesMut::new();
        let packet_id = crate::protocol::encode::packet_id::<P>(&self.state().await);
        let Some(packet_id) = packet_id else {
            panic!(
                "Failed to find packet id for Packet '{}'.",
                std::any::type_name_of_val(&packet)
            );
        };

        data.write_varint(packet_id).unwrap();
        P::encode(&mut data, &packet).unwrap();

        let mut swriter = self.swriter.lock().await;
        if let Err(_) = swriter.write_packet(&data.to_vec()).await {
            self.close();
        };
    }

    pub async fn kick(&self, reason: Component) {
        match *self.state.lock().await {
            ProtocolState::Play => self.send_packet(play::DisconnectPacket { reason }).await,
            _ => todo!(),
        }
    }

    pub fn closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }
}
