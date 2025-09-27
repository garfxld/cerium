use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("move_player_pos_rot", 0x1E)]
pub struct PlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8, // 0x01: on ground, 0x02: pushing against wall
}

impl ClientPacket for PlayerPositionAndRotationPacket {}

impl Decode for PlayerPositionAndRotationPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            x:      r.read_f64()?,
            feet_y: r.read_f64()?,
            z:      r.read_f64()?,
            yaw:    r.read_f32()?,
            pitch:  r.read_f32()?,
            flags:  r.read_u8()?,
        })
    }
}

impl Encode for PlayerPositionAndRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_f64(this.x)?;
        w.write_f64(this.feet_y)?;
        w.write_f64(this.z)?;
        w.write_f32(this.yaw)?;
        w.write_f32(this.pitch)?;
        w.write_u8(this.flags)?;
        Ok(())
    }
}
