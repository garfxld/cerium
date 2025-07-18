use bytes::BytesMut;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("status_request")]
pub struct StatusRequestPacket {
    // Empty
}

impl Decode for StatusRequestPacket {
    fn decode(_: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {})
    }
}
