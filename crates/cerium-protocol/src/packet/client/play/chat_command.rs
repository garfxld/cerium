use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("chat_command")]
pub struct ChatCommandPacket {
    pub command: String,
}

impl Decode for ChatCommandPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            command: buffer.read_string()?,
        })
    }
}
