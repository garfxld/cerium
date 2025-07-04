use std::sync::Arc;

use bytes::{Buf as _, BufMut as _, BytesMut};
use tokio::net::TcpStream;

use crate::protocol::{
    ProtcolState,
    decoder::Decode,
    encoder::Encode,
    packet::{
        HandshakePacket, PingRequestPacket, PongResponsePacket, StatusRequestPacket,
        StatusResponsePacket,
    },
    types::VarInt,
};

pub struct ClientConnection {
    stream: Arc<TcpStream>,
    state: ProtcolState,
}

impl ClientConnection {
    pub fn new(stream: Arc<TcpStream>) -> Self {
        Self {
            stream,
            state: ProtcolState::Handshake,
        }
    }

    pub async fn read(&mut self) -> anyhow::Result<()> {
        loop {
            self.stream.readable().await?;

            let mut buffer = vec![0; 32_767];

            match self.stream.try_read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => buffer.truncate(n),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e.into()),
            };

            let mut buffer = BytesMut::from(buffer.as_slice());
            let length = buffer.capacity() as i32;

            println!("received {} bytes", length);

            let buffer = &mut buffer;

            let mut read_bytes = 0;

            while buffer.remaining() > 0 {
                let packet_length = VarInt::decode(buffer)?.0;
                if read_bytes + packet_length > buffer.remaining() as i32 {
                    break;
                }

                let packet_id = VarInt::decode(buffer)?.0;

                match self.state {
                    ProtcolState::Handshake => self.handle_handshake(packet_id, buffer),
                    ProtcolState::Status => self.handle_status(packet_id, buffer),
                    _ => unimplemented!(),
                }

                read_bytes += packet_length + 1;
            }
        }
        Ok(())
    }

    fn handle_handshake(&mut self, id: i32, data: &mut BytesMut) {
        match id {
            0x00 => {
                let packet = HandshakePacket::decode(data).unwrap();
                println!("{:?}", packet);

                self.state = match packet.intent {
                    1 => ProtcolState::Status,
                    2 => ProtcolState::Login,
                    3 => unimplemented!("Not yet implemented."),
                    _ => panic!("Invalid next intent"),
                };
            }
            _ => panic!("Unknown packet! ({})", id),
        }
    }

    fn handle_status(&mut self, id: i32, data: &mut BytesMut) {
        match id {
            0x0 => {
                println!("{:?}", StatusRequestPacket::decode(data).unwrap());

                let packet = StatusResponsePacket {
                    json_response: r#"
                    {
                        "version": {
                            "name": "1.21.7",
                            "protocol": 772
                        },
                        "players": {
                            "max": 100,
                            "online": 5,
                            "sample": [
                                {
                                    "name": "thinkofdeath",
                                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                                }
                            ]
                        },
                        "description": {
                            "text": "Hello, world!"
                        },
                        "favicon": "data:image/png;base64,<data>",
                        "enforcesSecureChat": false
                    }
                    "#
                    .to_string(),
                };

                self.send_packet(0x00, packet);
            }
            0x01 => {
                let packet = PingRequestPacket::decode(data).unwrap();

                self.send_packet(
                    0x01,
                    PongResponsePacket {
                        timestamp: packet.timestamp,
                    },
                );
            }
            _ => panic!("Unknown packet! ({})", id),
        }
    }

    fn send_packet<P>(&mut self, packet_id: i32, packet: P)
    where
        P: Encode<P>,
    {
        let mut data = BytesMut::new();
        VarInt::encode(&mut data, VarInt(packet_id)).unwrap();
        P::encode(&mut data, packet).unwrap();

        let mut buffer = BytesMut::new();
        VarInt::encode(&mut buffer, VarInt(data.len() as i32)).unwrap();
        buffer.put(data);

        self.stream.try_write(&mut buffer).unwrap();
    }
}
