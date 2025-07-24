use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("status_request")]
pub struct StatusRequestPacket {
    // Empty
}

impl Decode for StatusRequestPacket {
    fn decode(_: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
