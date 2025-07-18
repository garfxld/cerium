use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("player_abilities")]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
}

impl Decode for PlayerAbilitiesPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            flags: buffer.read_i8()?,
        })
    }
}
