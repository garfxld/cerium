use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk
// Note: The order is inverted, because the client reads this packet as one big-endian Long, with Z being the upper 32 bits.
//       It is legal to send this packet even if the given chunk is not currently loaded.
#[derive(Debug, Clone)]
pub struct UnloadChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Packet for UnloadChunkPacket {}
impl ServerPacket for UnloadChunkPacket {}

impl Encode for UnloadChunkPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(this.chunk_z)?;
        w.write_i32(this.chunk_x)?;
        Ok(())
    }
}
