use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("container_close")]
pub struct CloseContainerPacket {
    pub window_id: i32,
}

impl Decode for CloseContainerPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(CloseContainerPacket {
            window_id: buffer.read_varint()?,
        })
    }
}
