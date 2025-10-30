pub mod heightmap;
pub mod palette;

use std::{collections::HashMap, sync::Arc};

mod chunk;
pub use chunk::Chunk;

mod chunk_section;
pub use chunk_section::ChunkSection;

mod block;
pub use block::{Block, BlockFace, BlockState};

mod block_entity;
pub use block_entity::BlockEntity;
use parking_lot::RwLock;

use crate::protocol::packet::{BlockUpdatePacket, WorldEventPacket};
use crate::registry::{DimensionType, REGISTRIES, RegistryKey};

use crate::entity::{Entity, Player};
use crate::util::BlockPosition;

#[derive(Clone)]
pub struct World(Arc<Inner>);

impl World {
    pub fn new(dimension: &RegistryKey<DimensionType>) -> Self {
        Self(Arc::new(Inner::new(dimension)))
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
        self.0.get_chunk(chunk_x, chunk_z)
    }

    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        self.0.load_chunk(chunk_x, chunk_z)
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> &BlockState {
        self.0.get_block(x, y, z)
    }

    pub fn set_block<B>(&self, x: i32, y: i32, z: i32, block: B)
    where
        B: AsRef<BlockState>,
    {
        self.0.set_block(x, y, z, block)
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        self.0.get_biome(x, y, z)
    }

    pub fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        self.0.set_biome(x, y, z, biome)
    }

    pub fn spawn_entity(&self, entity: Entity) {
        self.0.spawn_entity(entity)
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.0.entities()
    }

    pub fn break_block(&self, player: Player, position: BlockPosition, face: BlockFace) {
        self.0.break_block(player, position, face);
    }

    pub fn place_block(
        &self,
        player: Player,
        position: BlockPosition,
        face: BlockFace,
        block: BlockState,
    ) {
        self.0.place_block(player, position, face, block);
    }
}

struct Inner {
    dimension_type: DimensionType,
    chunks: RwLock<HashMap<(i32, i32), Chunk>>,
    entities: RwLock<Vec<Entity>>,
}

impl Inner {
    fn new(dimension: &RegistryKey<DimensionType>) -> Self {
        let dimension_type = REGISTRIES.dimension_type.get(dimension).unwrap().clone();

        Self {
            dimension_type,
            chunks: RwLock::new(HashMap::new()),
            entities: RwLock::new(Vec::new()),
        }
    }

    fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
        let chunks = self.chunks.read();
        chunks.get(&(chunk_x, chunk_z)).cloned()
    }

    fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut chunks = self.chunks.write();

        let chunk = Chunk::new(chunk_x, chunk_z, self.dimension_type.min_y);
        chunks.insert((chunk_x, chunk_z), chunk.clone());

        chunk
    }

    fn get_block(&self, x: i32, y: i32, z: i32) -> &BlockState {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        BlockState::from_id(chunk.get_block(x, y, z) as i32).unwrap()
    }

    fn set_block<B>(&self, x: i32, y: i32, z: i32, block: B)
    where
        B: AsRef<BlockState>,
    {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz),
        };
        chunk.set_block(x, y, z, block.as_ref());
    }

    fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
            panic!("Chunk ({},{}) is not loaded!", cx, cz);
        });

        chunk.get_biome(x, y, z)
    }

    fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        let cx = x / 16;
        let cz = z / 16;

        let chunk = match self.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => self.load_chunk(cx, cz),
        };
        chunk.set_biome(x, y, z, biome);
    }

    fn spawn_entity(&self, entity: Entity) {
        self.entities.write().push(entity);
    }

    fn entities(&self) -> Vec<Entity> {
        self.entities.read().iter().cloned().collect()
    }

    fn break_block(&self, player: Player, position: BlockPosition, _face: BlockFace) {
        // let (cx, cz) = Chunk::to_chunk_pos(position);
        // let Some(chunk) = self.get_chunk(cx, cz) else {
        //     return;
        // };

        let block = self.get_block(
            position.x() as i32,
            position.y() as i32,
            position.z() as i32,
        );
        self.set_block(
            position.x() as i32,
            position.y() as i32,
            position.z() as i32,
            Block::Air,
        );

        // todo: should be only sent to players that are viewing the block/chunk
        for ele in player.server().players.lock().clone() {
            ele.send_packet(BlockUpdatePacket {
                position,
                block_id: Block::Air.state_id(),
            });
            if ele == player {
                continue;
            }
            ele.send_packet(WorldEventPacket {
                event: 2001,
                position,
                data: block.id(),
                disable_relative_volume: false,
            });
        }
    }

    pub fn place_block(
        &self,
        player: Player,
        position: BlockPosition,
        face: BlockFace,
        block: BlockState,
    ) {
        let new_position = match face {
            BlockFace::Bottom => position.add(0, -1, 0),
            BlockFace::East => position.add(1, 0, 0),
            BlockFace::North => position.add(0, 0, -1),
            BlockFace::South => position.add(0, 0, 1),
            BlockFace::Top => position.add(0, 1, 0),
            BlockFace::West => position.add(-1, 0, 0),
        };

        self.set_block(
            new_position.x() as i32,
            new_position.y() as i32,
            new_position.z() as i32,
            &block,
        );

        // todo: should be only sent to players that are viewing the block/chunk
        for ele in player.server().players.lock().clone() {
            ele.send_packet(BlockUpdatePacket {
                position: new_position,
                block_id: block.state_id(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block() {
        let world = Inner::new(&DimensionType::OVERWORLD);

        world.load_chunk(0, 0);
        world.set_block(0, 0, 0, Block::MangrovePlanks);
        assert_eq!(world.get_block(0, 0, 0).state_id(), 26);
    }
}
