use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("chunk_batch_finished")]
pub struct ChunkBatchFinishedPacket {
    pub batch_size: i32,
}

impl Encode for ChunkBatchFinishedPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.batch_size)?;
        Ok(())
    }
}
