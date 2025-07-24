use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("finish_configuration")]
pub struct AcknowledgeFinishConfigPacket {
    // Empty
}

impl Decode for AcknowledgeFinishConfigPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
