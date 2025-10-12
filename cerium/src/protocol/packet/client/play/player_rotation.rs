use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

impl Packet for PlayerRotationPacket {}
impl ClientPacket for PlayerRotationPacket {}

impl Decode for PlayerRotationPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            yaw: r.read_f32()?,
            pitch: r.read_f32()?,
            flags: r.read_u8()?,
        })
    }
}
