use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
#[packet("set_chunk_cache_center")]
pub struct SetCenterChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Encode for SetCenterChunkPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_varint(this.chunk_x)?;
        buffer.write_varint(this.chunk_z)?;
        Ok(())
    }
}
