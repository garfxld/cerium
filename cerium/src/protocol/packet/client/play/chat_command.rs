use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ChatCommandPacket {
    pub command: String,
}

impl Packet for ChatCommandPacket {}
impl ClientPacket for ChatCommandPacket {}

impl Decode for ChatCommandPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            command: r.read_string()?,
        })
    }
}
