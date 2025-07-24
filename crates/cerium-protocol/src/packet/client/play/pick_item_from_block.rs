use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("pick_item_from_block")]
pub struct PickItemFromBlockPacket {
    pub position: i64, // todo: Position
    pub include_data: bool,
}

impl Decode for PickItemFromBlockPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            position: buffer.read_i64()?,
            include_data: buffer.read_bool()?,
        })
    }
}
