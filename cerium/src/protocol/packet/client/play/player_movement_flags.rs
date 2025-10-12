use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerMovementFlagsPacket {
    pub flags: u8,
}

impl Packet for PlayerMovementFlagsPacket {}
impl ClientPacket for PlayerMovementFlagsPacket {}

impl Decode for PlayerMovementFlagsPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_u8()?,
        })
    }
}
