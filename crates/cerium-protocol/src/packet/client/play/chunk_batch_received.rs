use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("chunk_batch_received")]
pub struct ChunkBatchReceivedPacket {
    pub chunks_per_tick: f32,
}

impl Decode for ChunkBatchReceivedPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            chunks_per_tick: buffer.read_f32()?,
        })
    }
}
