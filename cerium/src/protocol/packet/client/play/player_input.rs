use bitflags::bitflags;

use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerInputPacket {
    pub flags: PlayerInputFlags,
}

impl Packet for PlayerInputPacket {}
impl ClientPacket for PlayerInputPacket {}

impl Decode for PlayerInputPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: PlayerInputFlags::from_bits(r.read_u8()?).unwrap(),
        })
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct PlayerInputFlags: u8 {
        const FORWARD = 0x01;
        const BACKWARD = 0x02;
        const LEFT = 0x04;
        const RIGHT = 0x08;
        const JUMP = 0x10;
        const SNEAK = 0x20;
        const SPRINT = 0x40;
    }
}
