use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerLoadedPacket {
    // Empty
}

impl Packet for PlayerLoadedPacket {}
impl ClientPacket for PlayerLoadedPacket {}

impl Decode for PlayerLoadedPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
