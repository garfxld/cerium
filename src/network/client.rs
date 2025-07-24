use crate::{
    network::auth::{CryptContext, Decryptor, Encryptor, KeyStore},
    Server,
};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, BlockSizeUser as _};
use bytes::BytesMut;
use cerium_protocol::{buffer::ByteBuffer, encode::Encode, ProtocolState};
use cerium_util::auth::GameProfile;
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};
use tokio::sync::Mutex;
use uuid::Uuid;

// mutable
#[derive(Debug)]
pub struct ClientConnection {
    rstream: Mutex<OwnedReadHalf>,
    wstream: Mutex<OwnedWriteHalf>,
    addr: std::net::SocketAddr,
    pub state: Mutex<ProtocolState>,
    pub game_profile: Mutex<GameProfile>,
    pub key_store: Arc<KeyStore>,
    pub verify_token: Mutex<[u8; 4]>,
    pub crypt_context: Mutex<Option<CryptContext>>,
    pub closed: AtomicBool,
    pub server: Arc<Server>,
}

impl ClientConnection {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        server: Arc<Server>,
        key_store: Arc<KeyStore>,
    ) -> Self {
        let (rstream, wstream) = stream.into_split();
        Self {
            rstream: Mutex::new(rstream),
            wstream: Mutex::new(wstream),
            addr,
            state: Mutex::new(ProtocolState::Handshake),
            game_profile: Mutex::new(GameProfile {
                uuid: Uuid::new_v4(),
                name: String::new(),
                properties: vec![],
            }),
            key_store,
            verify_token: Mutex::new([0; 4]),
            crypt_context: Mutex::new(None),
            closed: AtomicBool::new(false),
            server,
        }
    }

    pub fn addr(&self) -> std::net::SocketAddr {
        self.addr
    }

    pub async fn read_loop(self: Arc<Self>) {
        let this = self.clone();
        while !this.closed.load(Ordering::Relaxed) {
            let rstream = this.rstream.lock().await;
            rstream.readable().await.unwrap();

            let mut buffer = vec![0; 32_767];

            match rstream.try_read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => buffer.truncate(n),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{}", e),
            };

            let mut buffer = BytesMut::from(buffer.as_slice());
            let length = buffer.capacity() as i32;

            if false {
                println!("received {} bytes", length);
            }

            if let Some(crypt_context) = &mut *this.crypt_context.lock().await {
                for block in buffer.chunks_mut(Decryptor::block_size()) {
                    crypt_context.decryptor.decrypt_block_mut(block.into());
                }
            }

            let buffer = &mut ByteBuffer::from(buffer);

            let mut read_bytes = 0;

            while buffer.remaining() > 0 {
                let packet_length = buffer.read_varint().unwrap();
                if read_bytes + packet_length > buffer.remaining() as i32 {
                    break;
                }
                let packet_id = buffer.read_varint().unwrap();

                let this = this.clone();
                this.handle_packet(packet_id, buffer).await.unwrap();

                read_bytes += packet_length + 1;
            }
        }
    }

    pub async fn send_packet<P>(&self, packet_id: i32, packet: P)
    where
        P: Encode + std::fmt::Debug,
    {
        log::trace!("{:?}", packet);

        let mut data = ByteBuffer::new();
        data.write_varint(packet_id).unwrap();
        P::encode(&mut data, packet).unwrap();

        let mut buffer = ByteBuffer::new();
        buffer.write_varint(data.len() as i32).unwrap();
        buffer.put(&*data.to_vec());

        let mut buffer: BytesMut = buffer.into();

        if let Some(crypt_context) = &mut *self.crypt_context.lock().await {
            for chunk in buffer.chunks_mut(Encryptor::block_size()) {
                crypt_context.encryptor.encrypt_block_mut(chunk.into());
            }
        }

        let wstream = self.wstream.lock().await;
        wstream.try_write(&mut buffer).unwrap();
    }
}
