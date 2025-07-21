use bytes::{Buf as _, BytesMut};
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("ping_request")]
pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl Decode for PingRequestPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            timestamp: buffer.read_i64()?,
        })
    }
}
