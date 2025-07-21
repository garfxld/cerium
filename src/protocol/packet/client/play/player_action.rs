use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("player_action")]
pub struct PlayerActionPacket {
    pub status: i32, // VarInt
    pub position: i64,
    pub face: u8,
    pub sequence: i32, // VarInt
}

impl Decode for PlayerActionPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            status: buffer.read_varint()?,
            position: buffer.read_i64()?,
            face: buffer.read_u8()?,
            sequence: buffer.read_varint()?,
        })
    }
}
