use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct StatusRequestPacket {
    // Empty
}

impl Packet for StatusRequestPacket {}
impl ClientPacket for StatusRequestPacket {}

impl Decode for StatusRequestPacket {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
