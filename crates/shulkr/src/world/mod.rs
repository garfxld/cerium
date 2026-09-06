pub mod attribute;
pub mod biome;
pub mod block;
pub mod chunk;
pub mod clock;
pub mod heightmap;
pub mod loader;
pub mod palette;
pub mod timeline;

mod dimension_type;

pub use dimension_type::*;

use crate::{
    entity::{Entity, Player},
    registry::RegistryKey,
    util::{BlockPosition, HashMap},
    world::{
        block::{BlockFace, BlockState},
        chunk::Chunk,
    },
};
use std::sync::Arc;

pub type ChunkGenerator = dyn Fn(&Chunk) + Send + Sync;

/// A Minecraft world.
#[derive(Clone)]
pub struct World(Arc<imp::World>);

impl World {
    pub fn new(dimension: RegistryKey<DimensionType>) -> Self {
        Self(Arc::new(imp::World::new(dimension, None)))
    }

    pub fn with_generator(
        dimension: RegistryKey<DimensionType>,
        generator: impl Fn(&Chunk) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(imp::World::new(
            dimension,
            Some(Arc::new(generator)),
        )))
    }

    pub fn dimension(&self) -> RegistryKey<DimensionType> {
        self.0.dimension()
    }

    pub fn set_chunk(&mut self, chunk: Chunk) {
        self.0.set_chunk(chunk);
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
        self.0.get_chunk(chunk_x, chunk_z)
    }

    pub fn chunk_count(&self) -> usize {
        self.0.chunk_count()
    }

    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        self.0.load_chunk(chunk_x, chunk_z)
    }

    pub async fn acquire_load_permit(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.0
            .load_semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("load_semaphore is never closed")
    }

    pub async fn load_chunk_async(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        if let Some(chunk) = self.0.get_chunk(chunk_x, chunk_z) {
            return chunk;
        }

        let (mut receiver, spawned_by_us) = self.0.take_or_create_pending(chunk_x, chunk_z);

        if spawned_by_us {
            let world = self.clone();
            tokio::task::spawn_blocking(move || {
                let chunk = world.0.load_chunk(chunk_x, chunk_z);
                world.0.finish_pending(chunk_x, chunk_z, chunk);
            });
        }

        loop {
            if let Some(chunk) = receiver.borrow_and_update().clone() {
                return chunk;
            }

            if receiver.changed().await.is_err() {
                return self.0.load_chunk(chunk_x, chunk_z);
            }
        }
    }

    pub(crate) fn add_viewer(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.0.add_viewer(chunk_x, chunk_z)
    }

    pub(crate) fn remove_viewer(&self, chunk_x: i32, chunk_z: i32) {
        self.0.remove_viewer(chunk_x, chunk_z);
    }

    pub fn get_block(&self, position: impl Into<BlockPosition>) -> BlockState {
        let position = position.into();
        self.0.get_block(position.x(), position.y(), position.z())
    }

    pub fn set_block(&self, position: impl Into<BlockPosition>, block: impl Into<BlockState>) {
        let position = position.into();
        self.0
            .set_block(position.x(), position.y(), position.z(), block)
    }

    pub fn get_biome(&self, position: impl Into<BlockPosition>) -> u16 {
        let position = position.into();
        self.0.get_biome(position.x(), position.y(), position.z())
    }

    pub fn set_biome(&self, position: impl Into<BlockPosition>, biome: i32) {
        let position = position.into();
        self.0
            .set_biome(position.x(), position.y(), position.z(), biome)
    }

    pub fn spawn_entity(&self, entity: Entity) {
        self.0.spawn_entity(entity)
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.0.entities()
    }

    pub fn break_block(&self, player: Player, position: impl Into<BlockPosition>, face: BlockFace) {
        self.0.break_block(player, position.into(), face);
    }

    pub fn place_block(
        &self,
        player: Player,
        position: impl Into<BlockPosition>,
        face: BlockFace,
        block: BlockState,
    ) {
        self.0.place_block(player, position.into(), face, block);
    }
}

mod imp {
    use super::*;
    use crate::{
        Server,
        protocol::packet::{BlockUpdatePacket, WorldEventPacket},
        util::RwLock,
        world::{block::Block, chunk::AsyncDedup},
    };
    use std::sync::OnceLock;
    use tokio::sync::{Semaphore, watch};

    #[derive(Default)]
    pub(super) struct ChunkHolder {
        chunk: Arc<OnceLock<Chunk>>,
        viewers: usize,
    }

    impl ChunkHolder {
        fn ready(&self) -> Option<&Chunk> {
            self.chunk.get()
        }
    }

    pub(super) struct World {
        dimension: RegistryKey<DimensionType>,
        dimension_type: DimensionType,
        pub(super) chunks: RwLock<HashMap<(i32, i32), ChunkHolder>>,
        entities: RwLock<Vec<Entity>>,
        generator: Option<Arc<super::ChunkGenerator>>,
        pending: AsyncDedup<(i32, i32), Chunk>,
        pub(super) load_semaphore: Arc<Semaphore>,
    }

    impl World {
        pub(super) fn new(
            dimension: RegistryKey<DimensionType>,
            generator: Option<Arc<super::ChunkGenerator>>,
        ) -> Self {
            let server = Server::current();
            let dimension_type = server
                .registries()
                .dimension_type
                .by_key(&dimension.to_key())
                .unwrap()
                .clone();

            Self {
                dimension,
                dimension_type,
                chunks: RwLock::new(HashMap::default()),
                entities: RwLock::new(Vec::new()),
                generator,
                pending: AsyncDedup::new(),
                load_semaphore: Arc::new(Semaphore::new(
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(4)
                        * 4,
                )),
            }
        }

        pub(super) fn dimension(&self) -> RegistryKey<DimensionType> {
            self.dimension.clone()
        }

        pub(super) fn take_or_create_pending(
            &self,
            chunk_x: i32,
            chunk_z: i32,
        ) -> (watch::Receiver<Option<Chunk>>, bool) {
            self.pending.take_or_create((chunk_x, chunk_z))
        }

        pub(super) fn finish_pending(&self, chunk_x: i32, chunk_z: i32, chunk: Chunk) {
            self.pending.finish((chunk_x, chunk_z), chunk);
        }

        pub(super) fn set_chunk(&self, chunk: Chunk) {
            let pos = (chunk.x(), chunk.z());
            let cell = Arc::new(OnceLock::new());
            let _ = cell.set(chunk);
            self.chunks.write().insert(
                pos,
                ChunkHolder {
                    chunk: cell,
                    viewers: 0,
                },
            );
        }

        pub(super) fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
            self.chunks
                .read()
                .get(&(chunk_x, chunk_z))
                .and_then(ChunkHolder::ready)
                .cloned()
        }

        pub(super) fn chunk_count(&self) -> usize {
            self.chunks
                .read()
                .values()
                .filter(|slot| slot.ready().is_some())
                .count()
        }

        pub(super) fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
            let cell = Arc::clone(
                &self
                    .chunks
                    .write()
                    .entry((chunk_x, chunk_z))
                    .or_default()
                    .chunk,
            );

            cell.get_or_init(|| {
                let chunk = Chunk::new(chunk_x, chunk_z, self.dimension_type.min_y);
                if let Some(generator) = &self.generator {
                    generator(&chunk);
                }
                chunk
            })
            .clone()
        }

        pub(super) fn add_viewer(&self, chunk_x: i32, chunk_z: i32) -> bool {
            let mut chunks = self.chunks.write();
            match chunks.get_mut(&(chunk_x, chunk_z)) {
                Some(slot) if slot.ready().is_some() => {
                    slot.viewers += 1;
                    true
                }
                _ => false,
            }
        }

        pub(super) fn remove_viewer(&self, chunk_x: i32, chunk_z: i32) {
            let mut chunks = self.chunks.write();
            if let std::collections::hash_map::Entry::Occupied(mut entry) =
                chunks.entry((chunk_x, chunk_z))
            {
                // A slot whose generator is still running has no viewer to drop
                // and must not be evicted out from under it.
                if entry.get().ready().is_none() {
                    return;
                }
                let slot = entry.get_mut();
                slot.viewers = slot.viewers.saturating_sub(1);
            }
        }

        pub(super) fn get_block(&self, x: i32, y: i32, z: i32) -> BlockState {
            let cx = x.div_euclid(16);
            let cz = z.div_euclid(16);

            let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
                panic!("Chunk ({},{}) is not loaded!", cx, cz);
            });

            BlockState::from_id(chunk.get_block(x, y, z)).unwrap()
        }

        pub(super) fn set_block<B>(&self, x: i32, y: i32, z: i32, block: B)
        where
            B: Into<BlockState>,
        {
            let cx = x.div_euclid(16);
            let cz = z.div_euclid(16);

            let chunk = match self.get_chunk(cx, cz) {
                Some(chunk) => chunk,
                None => self.load_chunk(cx, cz),
            };
            chunk.set_block(x, y, z, &block.into());
        }

        pub(super) fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
            let cx = x.div_euclid(16);
            let cz = z.div_euclid(16);

            let chunk = self.get_chunk(cx, cz).unwrap_or_else(|| {
                panic!("Chunk ({},{}) is not loaded!", cx, cz);
            });

            chunk.get_biome(x, y, z)
        }

        pub(super) fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
            let cx = x.div_euclid(16);
            let cz = z.div_euclid(16);

            let chunk = match self.get_chunk(cx, cz) {
                Some(chunk) => chunk,
                None => self.load_chunk(cx, cz),
            };
            chunk.set_biome(x, y, z, biome);
        }

        pub(super) fn spawn_entity(&self, entity: Entity) {
            self.entities.write().push(entity);
        }

        pub(super) fn entities(&self) -> Vec<Entity> {
            self.entities.read().iter().cloned().collect()
        }

        pub(super) fn break_block(
            &self,
            player: Player,
            position: BlockPosition,
            _face: BlockFace,
        ) {
            // let (cx, cz) = Chunk::to_chunk_pos(position);
            // let Some(chunk) = self.get_chunk(cx, cz) else {
            //     return;
            // };

            let block = self.get_block(position.x(), position.y(), position.z());
            let air = Block::AIR.default_state();
            self.set_block(position.x(), position.y(), position.z(), air);

            // todo: should be only sent to players that are viewing the block/chunk
            for p in player.server().players().lock().clone() {
                p.send_packet(&BlockUpdatePacket {
                    position,
                    block_id: air.state_id(),
                });
                if p == player {
                    continue;
                }
                p.send_packet(&WorldEventPacket {
                    event: 2001,
                    position,
                    data: block.state_id() as i32,
                    disable_relative_volume: false,
                });
            }
        }

        pub(super) fn place_block(
            &self,
            player: Player,
            position: BlockPosition,
            face: BlockFace,
            state: impl Into<BlockState>,
        ) {
            let state = state.into();
            let block_id = state.state_id();

            let new_position = position.relative(face);
            self.set_block(new_position.x(), new_position.y(), new_position.z(), state);

            // todo: should be only sent to players that are viewing the block/chunk
            for player in player.server().players().lock().clone() {
                player.send_packet(&BlockUpdatePacket {
                    position: new_position,
                    block_id,
                });
            }
        }
    }
}
