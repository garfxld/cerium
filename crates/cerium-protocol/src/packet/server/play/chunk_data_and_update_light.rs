use bytes::buf;
use cerium_protocol_macros::packet;
use cerium_registry::generated::block::Block;
use cerium_world::{chunk::BlockEntity, heightmap::Heightmap, light::LightData};

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

impl Encode for ChunkData {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_array(this.heightmaps, |buffer, value| {
            Heightmap::encode(buffer, value)
        })?;
        buffer.write_array(this.data, |buffer, value| buffer.write_u8(value))?;

        buffer.write_array(this.block_entities, |buffer, value| {
            BlockEntity::encode(buffer, value)
        });
        Ok(())
    }
}

impl Encode for BlockEntity {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_u8(this.packed_xz);
        buffer.write_i16(this.y);
        buffer.write_varint(this.r#type);

        let mut data: Vec<u8> = Vec::new();
        this.data.write_unnamed(&mut data);
        buffer.put(&*data);
        Ok(())
    }
}

impl Encode for Heightmap {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.kind)?;
        buffer.write_array(this.data, |buffer, value| buffer.write_i64(value))?;
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

impl Encode for cerium_world::palette::Palette {
    fn encode(buffer: &mut ByteBuffer, mut this: Self) -> Result<(), EncodeError> {
        this.compute();

        buffer.write_u8(this.bpe)?;
        cerium_world::palette::PaletteFormat::encode(buffer, this.format)?;
        for value in this.values {
            buffer.write_i64(value)?;
        }

        Ok(())
    }
}

impl Encode for cerium_world::palette::PaletteFormat {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        match this {
            cerium_world::palette::PaletteFormat::SingleValued { value } => {
                buffer.write_varint(value)?;
            }
            cerium_world::palette::PaletteFormat::Indirect { values } => {
                buffer.write_array(values, |buffer, value| buffer.write_varint(value))?;
            }
            cerium_world::palette::PaletteFormat::Direct => {}
        }
        Ok(())
    }
}

impl Into<ChunkDataAndUpdateLightPacket> for &cerium_world::chunk::Chunk {
    fn into(self) -> ChunkDataAndUpdateLightPacket {
        let mut data = ByteBuffer::new();
        for section in &self.sections {
            cerium_world::chunk_section::ChunkSection::encode(&mut data, section.clone()).unwrap();
        }

        let data = ChunkData {
            heightmaps: vec![],
            data: data.to_vec(),
            block_entities: self.block_entities.clone(),
        };

        let light = LightData {};

        ChunkDataAndUpdateLightPacket {
            chunk_x: self.chunk_x,
            chunk_z: self.chunk_z,
            data,
            light,
        }
    }
}

impl Encode for cerium_world::chunk_section::ChunkSection {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i16(this.block_states.count() as i16)?;
        cerium_world::palette::Palette::encode(buffer, this.block_states)?;
        cerium_world::palette::Palette::encode(buffer, this.biomes)?;
        Ok(())
    }
}
