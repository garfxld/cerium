use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct ChunkBatchFinishedPacket {
    pub batch_size: i32,
}

impl Packet for ChunkBatchFinishedPacket {}
impl ServerPacket for ChunkBatchFinishedPacket {}

impl Encode for ChunkBatchFinishedPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.batch_size)?;
        Ok(())
    }
}
