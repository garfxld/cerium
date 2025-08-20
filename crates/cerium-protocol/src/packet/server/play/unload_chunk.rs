use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("forget_level_chunk")]
pub struct UnloadChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Encode for UnloadChunkPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        // https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk
        // Note: The order is inverted, because the client reads this packet as one big-endian Long, with Z being the upper 32 bits.
        //       It is legal to send this packet even if the given chunk is not currently loaded.
        buffer.write_i32(this.chunk_z)?;
        buffer.write_i32(this.chunk_x)?;
        Ok(())
    }
}
