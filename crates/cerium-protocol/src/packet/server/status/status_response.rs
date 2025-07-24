use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("status_response")]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl Encode for StatusResponsePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_string(this.json_response)?;
        Ok(())
    }
}
