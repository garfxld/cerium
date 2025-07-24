use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
#[packet("finish_configuration")]
pub struct FinishConfigPacket {
    // Empty
}

impl Encode for FinishConfigPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
