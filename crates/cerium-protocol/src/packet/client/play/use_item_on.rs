use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("use_item_on")]
pub struct UseItemOnPacket {
    pub hand: i32, // VarInt Enum (Hand)
    pub position: i64,
    pub face: i32, // VarInt Enum?
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    pub sequence: i32, // VarInt
}

impl Decode for UseItemOnPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            hand: buffer.read_varint()?,
            position: buffer.read_i64()?,
            face: buffer.read_varint()?,
            cursor_x: buffer.read_f32()?,
            cursor_y: buffer.read_f32()?,
            cursor_z: buffer.read_f32()?,
            inside_block: buffer.read_bool()?,
            world_border_hit: buffer.read_bool()?,
            sequence: buffer.read_varint()?,
        })
    }
}
