use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug)]
pub struct CustomModelData {
    floats: Vec<f32>,
    flags: Vec<bool>,
    strings: Vec<String>,
    colors: Vec<i32>,
}

impl DataType for CustomModelData {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            floats: r.read_array(|r| r.read_f32())?,
            flags: r.read_array(|r| r.read_bool())?,
            strings: r.read_array(|r| r.read_string())?,
            colors: r.read_array(|r| r.read_i32())?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.floats, |w, v| w.write_f32(*v))?;
        w.write_array(&this.flags, |w, v| w.write_bool(*v))?;
        w.write_array(&this.strings, |w, v| w.write_string(v))?;
        w.write_array(&this.colors, |w, v| w.write_i32(*v))?;
        Ok(())
    }
}
