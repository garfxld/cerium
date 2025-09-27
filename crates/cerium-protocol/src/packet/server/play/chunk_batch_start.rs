use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("chunk_batch_start")]
pub struct ChunkBatchStartPacket {
    // Empty
}

impl Decode for ChunkBatchStartPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for ChunkBatchStartPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
