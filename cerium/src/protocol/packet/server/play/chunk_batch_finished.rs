use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("chunk_batch_finished", 0x0B)]
pub struct ChunkBatchFinishedPacket {
    pub batch_size: i32,
}

impl Decode for ChunkBatchFinishedPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            batch_size: r.read_varint()?,
        })
    }
}

impl Encode for ChunkBatchFinishedPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.batch_size)?;
        Ok(())
    }
}
