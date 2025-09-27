use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("set_chunk_cache_center")]
pub struct SetCenterChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Decode for SetCenterChunkPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            chunk_x: r.read_varint()?,
            chunk_z: r.read_varint()?,
        })
    }
}

impl Encode for SetCenterChunkPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.chunk_x)?;
        w.write_varint(this.chunk_z)?;
        Ok(())
    }
}
