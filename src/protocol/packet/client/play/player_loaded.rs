use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("player_loaded")]
pub struct PlayerLoadedPacket {
    // Empty
}

impl Decode for PlayerLoadedPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(PlayerLoadedPacket {})
    }
}
