use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("chunk_batch_start")]
pub struct ChunkBatchStartPacket {
    // Empty
}

impl Encode for ChunkBatchStartPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
