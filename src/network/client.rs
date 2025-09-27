use crate::{
    Server,
    entity::player::Player,
    network::{auth::KeyStore, reader::StreamReader, writer::StreamWriter},
};

use bytes::{BufMut, Bytes, BytesMut};
use cerium_protocol::{ProtocolState, encode::Encode, write::PacketWrite};
use cerium_util::auth::GameProfile;
use std::{
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

            let mut data = BytesMut::new();
            data.put_slice(&packet.data());
            let mut data = Bytes::from(data);

            this.handle_packet(packet.id(), &mut data).await.unwrap();
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

    pub async fn send_packet<P>(&self, packet_id: i32, packet: P)
    where
        P: Encode,
    {
        let mut data = BytesMut::new();
        data.write_varint(packet_id).unwrap();
        P::encode(&mut data, packet).unwrap();

        let mut swriter = self.swriter.lock().await;
        if let Err(_) = swriter.write_packet(&data.to_vec()).await {
            self.close();
        };
    }

    pub fn closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }
}
