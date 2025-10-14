use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct AcknowledgeFinishConfigPacket {
    // Empty
}

impl Packet for AcknowledgeFinishConfigPacket {}
impl ClientPacket for AcknowledgeFinishConfigPacket {}

impl Decode for AcknowledgeFinishConfigPacket {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
