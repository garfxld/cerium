pub mod chunk;
pub mod chunk_section;

pub mod heightmap;
pub mod palette;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use cerium_registry::{BlockState, DimensionType, REGISTRIES, RegistryKey};

use crate::chunk::Chunk;

pub struct World {
    dimension_type: DimensionType,
    chunks: RwLock<HashMap<(i32, i32), Arc<Mutex<Chunk>>>>,
}

impl World {
    pub fn new(dimension: &RegistryKey<DimensionType>) -> Arc<Self> {
        let dimension_type = REGISTRIES.dimension_type.get(dimension).unwrap().clone();

        Arc::new(Self {
            dimension_type,
            chunks: RwLock::new(HashMap::new()),
        })
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Arc<Mutex<Chunk>>> {
        let chunks = self.chunks.read().unwrap();
        chunks.get(&(chunk_x, chunk_z)).cloned()
    }

    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Arc<Mutex<Chunk>> {
        let mut chunks = self.chunks.write().unwrap();

        let chunk = Arc::new(Mutex::new(Chunk::new(
            chunk_x,
            chunk_z,
            self.dimension_type.min_y,
        )));
        chunks.insert((chunk_x, chunk_z), chunk.clone());

        chunk
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> &BlockState {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        BlockState::from_id(chunk.lock().unwrap().get_block(x, y, z) as i32).unwrap()
    }

    pub fn set_block<B>(&self, x: i32, y: i32, z: i32, block: B)
    where
        B: AsRef<BlockState>,
    {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz),
        };
        chunk.lock().unwrap().set_block(x, y, z, block.as_ref());
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        chunk.lock().unwrap().get_biome(x, y, z)
    }

    pub fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz),
        };
        chunk.lock().unwrap().set_biome(x, y, z, biome);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cerium_registry::Block;

    #[tokio::test]
    async fn test_get_block() {
        let world = World::new(&DimensionType::OVERWORLD);

        world.load_chunk(0, 0);
        world.set_block(0, 0, 0, Block::MangrovePlanks);
        assert_eq!(world.get_block(0, 0, 0).state_id(), 22);
    }
}
