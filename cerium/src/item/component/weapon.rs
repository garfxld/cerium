use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    damage_per_attack: i32,
    disable_blocking_for: f32,
}

impl DataType for Weapon {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            damage_per_attack: r.read_varint()?,
            disable_blocking_for: r.read_f32()?,
        })
    }
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.damage_per_attack)?;
        w.write_f32(this.disable_blocking_for)?;
        Ok(())
    }
}
