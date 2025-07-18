use bytes::BufMut;
use macros::packet;

use crate::{
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeException},
    },
    world::{heightmap::Heightmap, light::LightData},
};

#[derive(Debug)]
#[packet("level_chunk_with_light")]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: ChunkData,
    pub light: LightData,
}

impl Encode for ChunkDataAndUpdateLightPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_i32(this.chunk_x)?;
        buffer.write_i32(this.chunk_z)?;
        ChunkData::encode(buffer, this.data)?;
        LightData::encode(buffer, this.light)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ChunkData {
    pub heightmaps: Vec<Heightmap>,
    pub data: Vec<u8>,
    pub block_entities: Vec<i8>, // BlockEntity array
}

impl Encode for ChunkData {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_array(this.heightmaps, |buffer, value| {
            Heightmap::encode(buffer, value)
        })?;
        buffer.write_array(this.data, |buffer, value| buffer.write_u8(value))?;

        // Zero block entities (for now)
        buffer.write_varint(0)?;
        Ok(())
    }
}
