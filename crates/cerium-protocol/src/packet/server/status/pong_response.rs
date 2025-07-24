use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("pong_response")]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl Encode for PongResponsePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i64(this.timestamp)?;
        Ok(())
    }
}
