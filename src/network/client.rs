use crate::{
    network::{
        auth::{CryptContext, Decryptor, Encryptor, GameProfile, KeyStore},
        listener::PacketHandler,
    },
    protocol::{buffer::ByteBuffer, encode::Encode, ProtcolState},
};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, BlockSizeUser as _};
use bytes::BytesMut;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::net::TcpStream;
use uuid::Uuid;

// mutable
pub struct ClientConnection {
    stream: Arc<TcpStream>,
    pub state: ProtcolState,
    pub game_profile: GameProfile,
    pub key_store: Arc<KeyStore>,
    pub verify_token: [u8; 4],
    pub crypt_context: Option<CryptContext>,
    pub closed: AtomicBool,
}

impl ClientConnection {
    pub fn new(stream: Arc<TcpStream>, key_store: Arc<KeyStore>) -> Self {
        Self {
            stream,
            state: ProtcolState::Handshake,
            game_profile: GameProfile {
                uuid: Uuid::new_v4(),
                name: String::new(),
                properties: vec![],
            },
            key_store,
            verify_token: [0; 4],
            crypt_context: None,
            closed: AtomicBool::new(false),
        }
    }

    pub async fn read_loop(&mut self) {
        while !self.closed.load(Ordering::Relaxed) {
            self.stream.readable().await.unwrap();

            let mut buffer = vec![0; 32_767];

            match self.stream.try_read(&mut buffer) {
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

            if let Some(crypt_context) = &mut self.crypt_context {
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

                self.handle_packet(packet_id, buffer);

                read_bytes += packet_length + 1;
            }
        }
    }

    pub fn send_packet<P>(&mut self, packet_id: i32, packet: P)
    where
        P: Encode + std::fmt::Debug,
    {
        log::trace!("{:?}", packet);

        let mut data = ByteBuffer::new();
        data.write_varint(packet_id).unwrap();
        P::encode(&mut data, packet).unwrap();

        let mut buffer = ByteBuffer::new();
        buffer.write_varint(data.len() as i32).unwrap();
        buffer.put_self(data);

        let mut buffer: BytesMut = buffer.into();

        if let Some(crypt_context) = &mut self.crypt_context {
            for chunk in buffer.chunks_mut(Encryptor::block_size()) {
                crypt_context.encryptor.encrypt_block_mut(chunk.into());
            }
        }

        self.stream.try_write(&mut buffer).unwrap();
    }
}
