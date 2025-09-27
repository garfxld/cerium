use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
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
    pub sequence: i32,
}

impl ClientPacket for UseItemOnPacket {}

impl Decode for UseItemOnPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hand:             r.read_varint()?,
            position:         r.read_i64()?,
            face:             r.read_varint()?,
            cursor_x:         r.read_f32()?,
            cursor_y:         r.read_f32()?,
            cursor_z:         r.read_f32()?,
            inside_block:     r.read_bool()?,
            world_border_hit: r.read_bool()?,
            sequence:         r.read_varint()?,
        })
    }
}

impl Encode for UseItemOnPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.hand)?;
        w.write_i64(this.position)?;
        w.write_varint(this.face)?;
        w.write_f32(this.cursor_x)?;
        w.write_f32(this.cursor_y)?;
        w.write_f32(this.cursor_z)?;
        w.write_bool(this.inside_block)?;
        w.write_bool(this.world_border_hit)?;
        w.write_varint(this.sequence)?;
        Ok(())
    }
}
