use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug, Clone)]
#[packet("finish_configuration")]
pub struct FinishConfigPacket {
    // Empty
}

impl Encode for FinishConfigPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        Ok(())
    }
}
