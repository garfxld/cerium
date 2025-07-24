use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("set_carried_item")]
pub struct SetHeldItemPacket {
    pub slot: i16,
}

impl Decode for SetHeldItemPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            slot: buffer.read_i16()?,
        })
    }
}
