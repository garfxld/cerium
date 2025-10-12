use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl Packet for PingRequestPacket {}
impl ClientPacket for PingRequestPacket {}

impl Decode for PingRequestPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            timestamp: r.read_i64()?,
        })
    }
}
