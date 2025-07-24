use crate::chunk_section::ChunkSection;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    min_y: i32,
    pub sections: Vec<ChunkSection>,
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

    #[inline]
    fn to_relative(value: i32) -> usize {
        (value & 0x0F) as usize
    }

    #[inline]
    fn section_at(&self, y: i32) -> Option<&ChunkSection> {
        self.sections.get(((y - self.min_y) / 16) as usize)
    }

    #[inline]
    fn section_at_mut(&mut self, y: i32) -> Option<&mut ChunkSection> {
        self.sections.get_mut(((y - self.min_y) / 16) as usize)
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        let section = self.section_at(y);

        if let Some(section) = section {
            section.get_block(
                Self::to_relative(x),
                Self::to_relative(y),
                Self::to_relative(z),
            )
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: i32) {
        let section = self.section_at_mut(y);

        if let Some(section) = section {
            section.set_block(
                Self::to_relative(x),
                Self::to_relative(y),
                Self::to_relative(z),
                block,
            );
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let section = self.section_at(y);

        if let Some(section) = section {
            section.get_biome(
                Self::to_relative(x) / 4,
                Self::to_relative(y) / 4,
                Self::to_relative(z) / 4,
            )
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: i32) {
        let section = self.section_at_mut(y);

        if let Some(section) = section {
            section.set_biome(
                Self::to_relative(x) / 4,
                Self::to_relative(y) / 4,
                Self::to_relative(z) / 4,
                biome,
            );
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }
}
