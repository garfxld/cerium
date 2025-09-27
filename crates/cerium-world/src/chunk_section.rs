use crate::palette::Palette;

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub block_states: Palette,
    pub biomes: Palette,
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
