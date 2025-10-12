use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: i32,
}

impl Packet for HandshakePacket {}
impl ClientPacket for HandshakePacket {}

impl Decode for HandshakePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            protocol_version: r.read_varint()?,
            server_address: r.read_string()?,
            server_port: r.read_u16()?,
            intent: r.read_varint()?,
        })
    }
}
