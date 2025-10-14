use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct ChunkBatchStartPacket {
    // Empty
}

impl Packet for ChunkBatchStartPacket {}
impl ServerPacket for ChunkBatchStartPacket {}

impl Encode for ChunkBatchStartPacket {
    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
