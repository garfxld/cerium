pub mod chunk;
pub mod chunk_section;

pub mod heightmap;
pub mod light;
pub mod palette;

use std::collections::HashMap;

use cerium_registry::{DimensionType, REGISTRIES, RegistryKey};

use crate::chunk::Chunk;

#[derive(Clone)]
pub struct World {
    dimension_type: DimensionType,
    chunks: HashMap<(i32, i32), Chunk>,
}

#[allow(unused)]
impl World {
    pub fn new(dimension: &RegistryKey<DimensionType>) -> Self {
        let dimension_type = REGISTRIES.dimension_type.get(dimension).unwrap().clone();

        Self {
            dimension_type,
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<&Chunk> {
        self.chunks.get(&(chunk_x, chunk_z))
    }

    pub fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> Option<&mut Chunk> {
        self.chunks.get_mut(&(chunk_x, chunk_z))
    }

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunks.insert(
            (chunk_x, chunk_z),
            Chunk::new(chunk_x, chunk_z, self.dimension_type.min_y),
        );
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        let chunk_x = x / 16;
        let chunk_z = z / 16;

        let chunk = self.get_chunk(chunk_x, chunk_z).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", chunk_x, chunk_z);
        });

        chunk.get_block(x, y, z)
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: i32) {
        let chunk_x = x / 16;
        let chunk_z = z / 16;

        let chunk = self.get_chunk_mut(chunk_x, chunk_z).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", chunk_x, chunk_z);
        });

        chunk.set_block(x, y, z, block);
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let chunk_x = x / 16;
        let chunk_z = z / 16;

        let chunk = self.get_chunk(chunk_x, chunk_z).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", chunk_x, chunk_z);
        });

        chunk.get_biome(x, y, z)
    }

    pub fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: i32) {
        let chunk_x = x / 16;
        let chunk_z = z / 16;

        let chunk = self.get_chunk_mut(chunk_x, chunk_z).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", chunk_x, chunk_z);
        });

        chunk.set_biome(x, y, z, biome);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block() {
        let mut world = World::new(&DimensionType::OVERWORLD);

        world.load_chunk(0, 0);
        world.set_block(0, 0, 0, 22);
        assert_eq!(world.get_block(0, 0, 0), 22);
    }
}
