use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("chunk_batch_received", 0x0A)]
pub struct ChunkBatchReceivedPacket {
    pub chunks_per_tick: f32,
}

impl ClientPacket for ChunkBatchReceivedPacket {}

impl Decode for ChunkBatchReceivedPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            chunks_per_tick: r.read_f32()?,
        })
    }
}

impl Encode for ChunkBatchReceivedPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_f32(this.chunks_per_tick)?;
        Ok(())
    }
}
