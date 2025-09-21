use bytes::buf;
use cerium_protocol_macros::packet;
use cerium_registry::generated::block::Block;
use cerium_world::{chunk::BlockEntity, heightmap::Heightmap};

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
    types::BitSet,
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
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
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
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone)]
pub struct LightData {}

impl Encode for ChunkData {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_array(this.heightmaps, |b, v| Heightmap::encode(b, v))?;
        buffer.write_array(this.data, |b, v| b.write_u8(v))?;
        buffer.write_array(this.block_entities, |b, v| BlockEntity::encode(b, v));
        Ok(())
    }
}

impl Encode for LightData {
    fn encode(buffer: &mut ByteBuffer, _this: Self) -> Result<(), EncodeError> {
        let num_sections = 26;

        buffer.write_varint(1)?;
        buffer.write_u64(0x3FFFFFF_u64)?;

        buffer.write_varint(0)?;

        buffer.write_varint(0)?;
        buffer.write_varint(1)?;
        buffer.write_u64(0x3FFFFFF_u64)?;

        let light_array = vec![0xFF; 2048];
        buffer.write_varint(num_sections as i32)?;
        for _ in 0..num_sections {
            buffer.write_byte_array(&light_array)?;
        }
        buffer.write_varint(0)?;

        Ok(())
    }
}
