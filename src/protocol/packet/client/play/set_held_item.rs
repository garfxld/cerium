use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug, Clone)]
#[packet("set_carried_item")]
pub struct SetHeldItemPacket {
    pub slot: i16,
}

impl Decode for SetHeldItemPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            slot: buffer.read_i16()?,
        })
    }
}
