use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
#[packet("status_response")]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl Encode for StatusResponsePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_string(this.json_response)?;
        Ok(())
    }
}
