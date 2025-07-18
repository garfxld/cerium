use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("move_player_rot")]
pub struct PlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

impl Decode for PlayerRotationPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            yaw: buffer.read_f32()?,
            pitch: buffer.read_f32()?,
            flags: buffer.read_u8()?,
        })
    }
}
