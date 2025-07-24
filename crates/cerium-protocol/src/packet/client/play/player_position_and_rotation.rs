use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("move_player_pos_rot")]
pub struct PlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8, // 0x01: on ground, 0x02: pushing against wall
}

impl Decode for PlayerPositionAndRotationPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            x: buffer.read_f64()?,
            feet_y: buffer.read_f64()?,
            z: buffer.read_f64()?,
            yaw: buffer.read_f32()?,
            pitch: buffer.read_f32()?,
            flags: buffer.read_u8()?,
        })
    }
}
