use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerInputPacket {
    pub flags: u8,
}

impl Packet for PlayerInputPacket {}
impl ClientPacket for PlayerInputPacket {}

impl Decode for PlayerInputPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_u8()?,
        })
    }
}
