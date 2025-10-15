pub mod heightmap;
pub mod palette;

use std::{collections::HashMap, sync::Arc};

mod chunk;
pub use chunk::Chunk;

mod chunk_section;
pub use chunk_section::ChunkSection;

mod block;
pub use block::{Block, BlockState};

mod block_entity;
pub use block_entity::BlockEntity;
use parking_lot::{Mutex, RwLock};

use crate::registry::{DimensionType, REGISTRIES, RegistryKey};

use crate::entity::Entity;

pub struct World {
    dimension_type: DimensionType,
    chunks: RwLock<HashMap<(i32, i32), Arc<Mutex<Chunk>>>>,
    entities: RwLock<Vec<Arc<Entity>>>,
}

impl World {
    pub fn new(dimension: &RegistryKey<DimensionType>) -> Arc<Self> {
        let dimension_type = REGISTRIES.dimension_type.get(dimension).unwrap().clone();

        Arc::new(Self {
            dimension_type,
            chunks: RwLock::new(HashMap::new()),
            entities: RwLock::new(Vec::new()),
        })
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Arc<Mutex<Chunk>>> {
        let chunks = self.chunks.read();
        chunks.get(&(chunk_x, chunk_z)).cloned()
    }

    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Arc<Mutex<Chunk>> {
        let mut chunks = self.chunks.write();

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

        BlockState::from_id(chunk.lock().get_block(x, y, z) as i32).unwrap()
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
        chunk.lock().set_block(x, y, z, block.as_ref());
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        chunk.lock().get_biome(x, y, z)
    }

    pub fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz),
        };
        chunk.lock().set_biome(x, y, z, biome);
    }

    pub fn spawn_entity(&self, entity: Arc<Entity>) {
        self.entities.write().push(entity);
    }

    pub fn entities(&self) -> Vec<Arc<Entity>> {
        self.entities.read().iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block() {
        let world = World::new(&DimensionType::OVERWORLD);

        world.load_chunk(0, 0);
        world.set_block(0, 0, 0, Block::MangrovePlanks);
        assert_eq!(world.get_block(0, 0, 0).state_id(), 26);
    }
}
