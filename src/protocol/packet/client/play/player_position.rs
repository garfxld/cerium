use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("move_player_pos")]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: u8, // 0x01: on ground, 0x02: pushing against wall
}

impl Decode for PlayerPositionPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            x: buffer.read_f64()?,
            feet_y: buffer.read_f64()?,
            z: buffer.read_f64()?,
            flags: buffer.read_u8()?,
        })
    }
}
