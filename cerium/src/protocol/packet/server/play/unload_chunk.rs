use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk
// Note: The order is inverted, because the client reads this packet as one big-endian Long, with Z being the upper 32 bits.
//       It is legal to send this packet even if the given chunk is not currently loaded.
#[derive(Debug, Clone)]
#[packet("forget_level_chunk", 0x21)]
pub struct UnloadChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Decode for UnloadChunkPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let chunk_z = r.read_i32()?;
        let chunk_x = r.read_i32()?;
        Ok(Self { chunk_x, chunk_z })
    }
}

impl Encode for UnloadChunkPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i32(this.chunk_z)?;
        w.write_i32(this.chunk_x)?;
        Ok(())
    }
}
