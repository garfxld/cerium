use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::{
    Server,
    auth::GameProfile,
    entity::{Entity, EntityType, GameMode},
    inventory::PlayerInventory,
    network::client::ClientConnection,
    protocol::packet::{
        ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket,
        GameEventPacket, Packet, ServerPacket, SystemChatMessagePacket, UnloadChunkPacket,
        server::play::KeepAlivePacket,
    },
    text::Component,
    tickable::Tickable,
    util::Position,
    world::{Chunk, World},
};

type SyncChunk = Arc<Mutex<Chunk>>;

pub struct ChunkQueue {
    queue: VecDeque<SyncChunk>,
    chunks_per_tick: i32,
}

impl ChunkQueue {
    pub fn new() -> Self {
        ChunkQueue {
            queue: VecDeque::new(),
            chunks_per_tick: 16,
        }
    }

    pub fn set_cpt(&mut self, chunks_per_tick: i32) {
        self.chunks_per_tick = chunks_per_tick;
    }

    pub fn queue(&mut self, chunk: SyncChunk) {
        self.queue.push_back(chunk);
    }

    pub fn drain(&mut self) -> Box<[SyncChunk]> {
        let size = self.queue.len().min(self.chunks_per_tick as usize);
        let chunks: Vec<SyncChunk> = self.queue.drain(0..size).collect();
        chunks.into_boxed_slice()
    }
}

pub struct Player {
    connection: Arc<ClientConnection>,
    game_profile: GameProfile,
    entity: Arc<Entity>,
    world: Mutex<Option<Arc<World>>>,
    position: Mutex<Option<Position>>,
    pub(crate) chunk_queue: tokio::sync::Mutex<ChunkQueue>,
    last_keep_alive: tokio::sync::Mutex<Instant>,
    inventory: Arc<PlayerInventory>,
    game_mode: Mutex<GameMode>,
}

impl Player {
    pub async fn new(connection: Arc<ClientConnection>, _server: Arc<Server>) -> Self {
        let game_profile = connection.game_profile.lock().await.clone().unwrap();
        Self {
            connection,
            game_profile,
            entity: Entity::new(EntityType::Player),
            world: Mutex::new(None),
            position: Mutex::new(None),
            chunk_queue: tokio::sync::Mutex::new(ChunkQueue::new()),
            last_keep_alive: tokio::sync::Mutex::new(Instant::now()),
            inventory: Arc::new(PlayerInventory::new()),
            game_mode: Mutex::new(GameMode::Survival),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn name(&self) -> &String {
        &self.game_profile.name
    }

    pub fn uuid(&self) -> Uuid {
        self.game_profile.uuid
    }

    pub fn world(&self) -> Arc<World> {
        self.world.lock().unwrap().clone().unwrap()
    }

    pub fn position(&self) -> Position {
        (*self.position.lock().unwrap()).unwrap()
    }

    pub fn game_mode(&self) -> GameMode {
        *self.game_mode.lock().unwrap()
    }

    pub fn id(&self) -> i32 {
        self.entity.id()
    }

    pub(crate) fn set_world(&self, world: Arc<World>) {
        (*self.world.lock().unwrap()) = Some(world)
    }

    pub(crate) fn set_position(&self, position: Position) {
        (*self.position.lock().unwrap()) = Some(position)
    }

    pub async fn set_game_mode(&self, game_mode: GameMode) {
        *self.game_mode.lock().unwrap() = game_mode;

        self.send_packet(GameEventPacket {
            event: 4,
            value: game_mode as i32 as f32,
        })
        .await;

        // todo: update client abilites
    }

    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.inventory
    }

    pub async fn send_message(&self, message: impl Into<Component>) {
        self.send_packet(SystemChatMessagePacket {
            content: message.into(),
            overlay: false,
        })
        .await;
    }

    pub async fn kick(&self, reason: impl Into<Component>) {
        self.connection.kick(reason.into()).await;
    }

    pub async fn send_packet<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        log::debug!("sending: {}", std::any::type_name_of_val(&packet));
        self.connection.send_packet(packet).await;
    }

    pub(crate) async fn load_chunks(&self) {
        let chunk = Chunk::to_chunk_pos(self.position());
        let view_distance = 32;

        let world = self.world();
        let chunks = Chunk::chunks_in_range(chunk, view_distance);
        println!("loading {:?} chunks", chunks.len());
        for (cx, cz) in chunks {
            let chunk = match world.get_chunk(cx, cz) {
                Some(chunk) => chunk,
                None => world.load_chunk(cx, cz),
            };

            self.send_chunk(chunk).await;
        }
    }

    pub(crate) async fn update_chunks(&self, new_chunk: (i32, i32), old_chunk: (i32, i32)) {
        let view_distance = 32;

        Chunk::difference(new_chunk, old_chunk, view_distance, async |cx, cz| {
            self.load_chunk(cx, cz).await;
        })
        .await;

        Chunk::difference(old_chunk, new_chunk, view_distance, async |cx, cz| {
            self.unload_chunk(cx, cz).await;
        })
        .await;
    }

    async fn load_chunk(&self, cx: i32, cz: i32) {
        let world = self.world();

        let chunk = match world.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => world.load_chunk(cx, cz),
        };

        self.send_chunk(chunk).await;
    }

    async fn unload_chunk(&self, cx: i32, cz: i32) {
        self.send_packet(UnloadChunkPacket {
            chunk_x: cx,
            chunk_z: cz,
        })
        .await;
    }

    async fn send_chunk(&self, chunk: SyncChunk) {
        self.chunk_queue.lock().await.queue(chunk);
    }

    // works for now
    async fn flush_chunks(&self) {
        let mut queue = self.chunk_queue.lock().await;

        let chunks = queue.drain();
        let size = chunks.len();

        if size < 1 {
            return;
        }
        self.send_packet(ChunkBatchStartPacket {}).await;

        for chunk in chunks {
            let packet: ChunkDataAndUpdateLightPacket = {
                let chunk = &*chunk.lock().unwrap();
                chunk.into()
            };
            self.send_packet(packet).await;
        }
        self.send_packet(ChunkBatchFinishedPacket {
            batch_size: size as i32,
        })
        .await;
    }

    async fn keep_alive(&self) {
        self.send_packet(KeepAlivePacket { keep_alive_id: 0 }).await
    }
}

impl Tickable for Player {
    async fn tick(self: &Arc<Self>) {
        let mut last_keep_alive = self.last_keep_alive.lock().await;
        if last_keep_alive.elapsed() > Duration::from_secs(20) {
            self.keep_alive().await;
            *last_keep_alive = Instant::now();
        }

        self.flush_chunks().await;
    }
}
