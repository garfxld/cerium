use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("player_loaded")]
pub struct PlayerLoadedPacket {
    // Empty
}

impl Decode for PlayerLoadedPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(PlayerLoadedPacket {})
    }
}
