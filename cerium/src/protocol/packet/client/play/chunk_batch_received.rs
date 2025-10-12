use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ChunkBatchReceivedPacket {
    pub chunks_per_tick: f32,
}

impl Packet for ChunkBatchReceivedPacket {}
impl ClientPacket for ChunkBatchReceivedPacket {}

impl Decode for ChunkBatchReceivedPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            chunks_per_tick: r.read_f32()?,
        })
    }
}
