use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
#[packet("pong_response")]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl Encode for PongResponsePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_i64(this.timestamp)?;
        Ok(())
    }
}
