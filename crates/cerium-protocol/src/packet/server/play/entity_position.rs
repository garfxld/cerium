use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("move_entity_pos", 0x2E)]
pub struct EntityPositionPacket {
    pub entitiy_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl Decode for EntityPositionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entitiy_id: r.read_varint()?,
            delta_x:    r.read_i16()?,
            delta_y:    r.read_i16()?,
            delta_z:    r.read_i16()?,
            on_ground:  r.read_bool()?,
        })
    }
}

impl Encode for EntityPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.entitiy_id)?;
        w.write_i16(this.delta_x)?;
        w.write_i16(this.delta_y)?;
        w.write_i16(this.delta_z)?;
        w.write_bool(this.on_ground)?;
        Ok(())
    }
}
