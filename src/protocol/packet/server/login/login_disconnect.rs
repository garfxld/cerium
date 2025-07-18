use macros::packet;
use uuid::Uuid;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug, Clone)]
#[packet("login_disconnect")]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl Encode for LoginDisconnectPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_string(this.reason)?;
        Ok(())
    }
}
