use bytes::Buf;
use macros::packet;

use crate::protocol::{buffer::ByteBuffer, decode::{Decode, DecodeException}};

#[derive(Debug)]
#[packet("player_input")]
pub struct PlayerInputPacket {
    pub flags: u8,
}

impl Decode for PlayerInputPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            flags: buffer.read_u8()?,
        })
    }
}
