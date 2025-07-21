use crate::{
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeError},
        packet::{ChunkData, ChunkDataAndUpdateLightPacket},
    },
    world::{light::LightData, palette::Palette},
};

#[derive(Debug, Clone)]
pub struct Chunk {
    chunk_x: i32,
    chunk_z: i32,
    min_y: i32,
    sections: Vec<ChunkSection>,
}

impl Chunk {
    pub fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
        let mut sections = vec![];
        for _ in 0..24 {
            sections.push(ChunkSection::new());
        }

        Self {
            chunk_x,
            chunk_z,
            min_y,
            sections,
        }
    }

    // x, z are relative
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: i32) {
        let section_y = (y - self.min_y) / 16;
        let section = self.sections.get_mut(section_y as usize);

        if let Some(section) = section {
            section.set_block(x, y, z, block);
        }
    }
}

impl Into<ChunkDataAndUpdateLightPacket> for Chunk {
    fn into(self) -> ChunkDataAndUpdateLightPacket {
        let mut data = ByteBuffer::new();
        for section in self.sections {
            ChunkSection::encode(&mut data, section).unwrap();
        }

        let data = ChunkData {
            heightmaps: vec![],
            data: data.to_vec(),
            block_entities: vec![],
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

#[derive(Debug, Clone)]
pub struct ChunkSection {
    block_count: i16,
    block_states: Palette,
    biomes: Palette,
}

impl ChunkSection {
    fn new() -> Self {
        Self {
            block_count: 4096,
            block_states: Palette::blocks(),
            biomes: Palette::biomes(),
        }
    }

    // xyz are relative
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: i32) {
        self.block_states.set(x, y, z, block);
        self.block_count = self.block_states.count() as i16;
    }
}

impl Encode for ChunkSection {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i16(this.block_count)?;
        Palette::encode(buffer, this.block_states)?;
        Palette::encode(buffer, this.biomes)?;
        Ok(())
    }
}
