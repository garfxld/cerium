use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("login_acknowledged")]
pub struct LoginAcknowledgePacket {
    // Empty
}

impl Decode for LoginAcknowledgePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
