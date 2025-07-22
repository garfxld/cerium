use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("move_player_status_only")]
pub struct PlayerMovementFlagsPacket {
    pub flags: u8,
}

impl Decode for PlayerMovementFlagsPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: buffer.read_u8()?,
        })
    }
}
