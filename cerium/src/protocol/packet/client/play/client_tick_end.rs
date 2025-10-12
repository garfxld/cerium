use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ClientTickEndPacket {
    // Empty
}

impl Packet for ClientTickEndPacket {}
impl ClientPacket for ClientTickEndPacket {}

impl Decode for ClientTickEndPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
