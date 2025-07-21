use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("player_abilities")]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
}

impl Decode for PlayerAbilitiesPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: buffer.read_i8()?,
        })
    }
}
