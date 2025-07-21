use bytes::Buf;
use macros::packet;
use uuid::Uuid;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("hello")]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl Decode for LoginStartPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            name: buffer.read_string()?,
            uuid: buffer.read_uuid()?,
        })
    }
}
