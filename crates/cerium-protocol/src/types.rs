use cerium_util::auth::Property;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

impl Decode for Property {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            name: r.read_string()?,
            value: r.read_string()?,
            signature: r.read_option(|r| r.read_string())?,
        })
    }
}

impl Encode for Property {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.name)?;
        w.write_string(this.value)?;
        w.write_option(this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}
