use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("login_compression", 0x03)]
pub struct SetCompressionPacket {
    pub threshold: i32,
}

impl Decode for SetCompressionPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            threshold: r.read_varint()?,
        })
    }
}

impl Encode for SetCompressionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.threshold)?;
        Ok(())
    }
}
