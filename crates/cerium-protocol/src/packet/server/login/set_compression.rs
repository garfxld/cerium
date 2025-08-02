use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
#[packet("login_compression")]
pub struct SetCompressionPacket {
    pub threshold: i32,
}

impl Encode for SetCompressionPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.threshold)?;
        Ok(())
    }
}
