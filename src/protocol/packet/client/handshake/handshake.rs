use bytes::{Buf as _, BytesMut};
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("intention")]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: i32,
}

impl Decode for HandshakePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            protocol_version: buffer.read_varint()?,
            server_address: buffer.read_string()?,
            server_port: buffer.read_u16()?,
            intent: buffer.read_varint()?,
        })
    }
}
