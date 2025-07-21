use bytes::BufMut;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
#[packet("hello")]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Box<[u8]>,
    pub verify_token: Box<[u8]>,
    pub should_authenticate: bool,
}

impl Encode for EncryptionRequestPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_string(this.server_id)?;
        buffer.write_array(this.public_key.to_vec(), |buffer, value| {
            buffer.write_u8(value)
        })?;
        buffer.write_array(this.verify_token.to_vec(), |buffer, value| {
            buffer.write_u8(value)
        })?;
        buffer.write_bool(this.should_authenticate)?;
        Ok(())
    }
}
