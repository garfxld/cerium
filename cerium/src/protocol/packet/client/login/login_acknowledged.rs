use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct LoginAcknowledgePacket {
    // Empty
}

impl Packet for LoginAcknowledgePacket {}
impl ClientPacket for LoginAcknowledgePacket {}

impl Decode for LoginAcknowledgePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
