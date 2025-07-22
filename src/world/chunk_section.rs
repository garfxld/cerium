use crate::{
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeError},
    },
    world::palette::Palette,
};

#[derive(Debug, Clone)]
pub struct ChunkSection {
    block_states: Palette,
    biomes: Palette,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            block_states: Palette::blocks(),
            biomes: Palette::biomes(),
        }
    }

    // Block

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u16 {
        self.block_states.get(x, y, z)
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: i32) {
        self.block_states.set(x, y, z, block as u16);
    }

    // Biome

    #[inline]
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: i32) {
        self.biomes.set(x, y, z, biome as u16);
    }

    #[inline]
    pub fn get_biome(&self, x: usize, y: usize, z: usize) -> u16 {
        self.biomes.get(x, y, z)
    }
}

impl Encode for ChunkSection {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i16(this.block_states.count() as i16)?;
        Palette::encode(buffer, this.block_states)?;
        Palette::encode(buffer, this.biomes)?;
        Ok(())
    }
}
