use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("player_command")]
pub struct PlayerCommandPacket {
    pub entity_id: i32,  // VarInt
    pub action_id: i32,  // todo: VarInt Enum
    pub jump_boost: i32, // VarInt
}

impl Decode for PlayerCommandPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id: buffer.read_varint()?,
            action_id: buffer.read_varint()?,
            jump_boost: buffer.read_varint()?,
        })
    }
}
