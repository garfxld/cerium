use serde::{Deserialize, Serialize};

use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Food {
    nutrition: i32,
    saturation_modifier: f32,
    can_always_eat: bool,
}

impl DataType for Food {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            nutrition: r.read_varint()?,
            saturation_modifier: r.read_f32()?,
            can_always_eat: r.read_bool()?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.nutrition)?;
        w.write_f32(this.saturation_modifier)?;
        w.write_bool(this.can_always_eat)?;
        Ok(())
    }
}
