use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: u8, // 0x01: on ground, 0x02: pushing against wall
}

impl Packet for PlayerPositionPacket {}
impl ClientPacket for PlayerPositionPacket {}

impl Decode for PlayerPositionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            x:      r.read_f64()?,
            feet_y: r.read_f64()?,
            z:      r.read_f64()?,
            flags:  r.read_u8()?,
        })
    }
}
