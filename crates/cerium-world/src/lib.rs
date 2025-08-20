pub mod chunk;
pub mod chunk_section;

pub mod heightmap;
pub mod light;
pub mod palette;

use std::{collections::HashMap, sync::Arc};

use cerium_registry::{DimensionType, REGISTRIES, RegistryKey};
use tokio::sync::{Mutex, RwLock};

use crate::chunk::Chunk;

pub struct World {
    dimension_type: DimensionType,
    chunks: RwLock<HashMap<(i32, i32), Arc<Mutex<Chunk>>>>,
}

#[allow(unused)]
impl World {
    pub fn new(dimension: &RegistryKey<DimensionType>) -> Self {
        let dimension_type = REGISTRIES.dimension_type.get(dimension).unwrap().clone();

        Self {
            dimension_type,
            chunks: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Arc<Mutex<Chunk>>> {
        let chunks = self.chunks.read().await;
        chunks.get(&(chunk_x, chunk_z)).cloned()
    }

    pub async fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Arc<Mutex<Chunk>> {
        let mut chunks = self.chunks.write().await;

        let chunk = Arc::new(Mutex::new(Chunk::new(
            chunk_x,
            chunk_z,
            self.dimension_type.min_y,
        )));
        chunks.insert((chunk_x, chunk_z), chunk.clone());

        chunk
    }

    pub async fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).await.unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        chunk.lock().await.get_block(x, y, z)
    }

    pub async fn set_block(&self, x: i32, y: i32, z: i32, block: i32) {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz).await {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz).await,
        };
        chunk.lock().await.set_block(x, y, z, block);
    }

    pub async fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).await.unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        chunk.lock().await.get_biome(x, y, z)
    }

    pub async fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz).await {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz).await,
        };
        chunk.lock().await.set_biome(x, y, z, biome);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block() {
        let world = World::new(&DimensionType::OVERWORLD);

        world.load_chunk(0, 0).await;
        world.set_block(0, 0, 0, 22).await;
        assert_eq!(world.get_block(0, 0, 0).await, 22);
    }
}
