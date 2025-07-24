use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("container_close")]
pub struct CloseContainerPacket {
    pub window_id: i32,
}

impl Decode for CloseContainerPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(CloseContainerPacket {
            window_id: buffer.read_varint()?,
        })
    }
}
