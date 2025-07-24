use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("client_tick_end")]
pub struct ClientTickEndPacket {
    // Empty
}

impl Decode for ClientTickEndPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(ClientTickEndPacket {})
    }
}
