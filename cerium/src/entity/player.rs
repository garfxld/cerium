use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::{
    Server,
    auth::GameProfile,
    entity::{Entity, EntityType, GameMode},
    inventory::PlayerInventory,
    network::client::Connection,
    protocol::packet::{
        ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket, GameEventPacket, Packet,
        ServerPacket, SystemChatMessagePacket, UnloadChunkPacket, server::play::KeepAlivePacket,
    },
    text::Component,
    tickable::Tickable,
    util::Position,
    world::{Chunk, World},
};

type SyncChunk = Arc<Mutex<Chunk>>;

pub struct ChunkQueue {
    pub queue: VecDeque<SyncChunk>,
    pub target_cpt: f32,
    pub pending_chunks: f32,
    pub max_lead: i32,
    pub lead: i32,
}

impl ChunkQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            target_cpt: 9.,
            pending_chunks: 0.,
            max_lead: 1,
            lead: 0,
        }
    }

    pub fn enqueue(&mut self, chunk: SyncChunk) {
        self.queue.push_back(chunk);
    }

    pub fn dequeue(&mut self) -> Option<SyncChunk> {
        self.queue.pop_front()
    }
}

pub struct Player {
    connection: Arc<Connection>,
    game_profile: GameProfile,
    entity: Arc<Entity>,
    world: Mutex<Option<Arc<World>>>,
    position: Mutex<Option<Position>>,
    last_keep_alive: Mutex<Instant>,
    inventory: Arc<PlayerInventory>,
    game_mode: Mutex<GameMode>,
    pub(crate) chunk_queue: Mutex<ChunkQueue>,
    // teleport_id: AtomicI32,
}

impl Player {
    pub async fn new(connection: Arc<Connection>, _server: Arc<Server>) -> Self {
        let game_profile = connection.game_profile.lock().clone().unwrap();
        Self {
            connection,
            game_profile,
            entity: Entity::new(EntityType::Player),
            world: Mutex::new(None),
            position: Mutex::new(None),
            last_keep_alive: Mutex::new(Instant::now()),
            inventory: Arc::new(PlayerInventory::new()),
            game_mode: Mutex::new(GameMode::Survival),
            chunk_queue: Mutex::new(ChunkQueue::new()),
            // teleport_id: AtomicI32::default(),
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
        self.world.lock().clone().unwrap()
    }

    pub fn position(&self) -> Position {
        (*self.position.lock()).unwrap()
    }

    pub fn game_mode(&self) -> GameMode {
        *self.game_mode.lock()
    }

    pub fn id(&self) -> i32 {
        self.entity.id()
    }

    pub(crate) fn set_world(&self, world: Arc<World>) {
        (*self.world.lock()) = Some(world)
    }

    pub(crate) fn set_position(&self, position: Position) {
        (*self.position.lock()) = Some(position)
    }

    pub async fn set_game_mode(&self, game_mode: GameMode) {
        *self.game_mode.lock() = game_mode;

        self.send_packet(GameEventPacket {
            event: 4,
            value: game_mode as i32 as f32,
        });

        // todo: update client abilites
    }

    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.inventory
    }

    pub async fn send_message(&self, message: impl Into<Component>) {
        self.send_packet(SystemChatMessagePacket {
            content: message.into(),
            overlay: false,
        });
    }

    pub fn kick(&self, reason: impl Into<Component>) {
        self.connection.kick(reason.into());
    }

    pub fn send_packet<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        self.connection.send_packet(packet);
    }

    pub(crate) fn load_chunks(&self) {
        let chunk = Chunk::to_chunk_pos(self.position());
        let view_distance = 32;

        let world = self.world();
        let chunks = Chunk::chunks_in_range(chunk, view_distance);

        for (cx, cz) in chunks {
            let chunk = match world.get_chunk(cx, cz) {
                Some(chunk) => chunk,
                None => world.load_chunk(cx, cz),
            };

            self.send_chunk(chunk);
        }

        self.send_pending_chunks();
    }

    pub(crate) fn update_chunks(&self, new_chunk: (i32, i32), old_chunk: (i32, i32)) {
        let view_distance = 32;

        Chunk::difference(new_chunk, old_chunk, view_distance, |cx, cz| {
            self.load_chunk(cx, cz);
        });

        Chunk::difference(old_chunk, new_chunk, view_distance, |cx, cz| {
            self.unload_chunk(cx, cz);
        });
    }

    fn load_chunk(&self, cx: i32, cz: i32) {
        let world = self.world();

        let chunk = match world.get_chunk(cx, cz) {
            Some(chunk) => chunk,
            None => world.load_chunk(cx, cz),
        };

        self.send_chunk(chunk);
    }

    fn unload_chunk(&self, cx: i32, cz: i32) {
        self.send_packet(UnloadChunkPacket {
            chunk_x: cx,
            chunk_z: cz,
        });
    }

    fn send_chunk(&self, chunk: SyncChunk) {
        let mut queue = self.chunk_queue.lock();
        queue.enqueue(chunk);
    }

    fn send_pending_chunks(&self) {
        let mut queue = self.chunk_queue.lock();

        if queue.queue.is_empty() || queue.lead >= queue.max_lead {
            return;
        }

        queue.pending_chunks = (queue.pending_chunks + queue.target_cpt).min(64.);
        if queue.pending_chunks < 1. {
            return;
        }

        self.connection.send_packet(ChunkBatchStartPacket {});

        // let mut batch_size = 0;
        while queue.pending_chunks >= 1.
            && let Some(chunk) = queue.dequeue()
        {
            let packet: ChunkDataAndUpdateLightPacket = {
                let chunk = &*chunk.lock();
                chunk.into()
            };

            self.send_packet(packet);

            queue.pending_chunks -= 1.;
            // batch_size += 1;
        }

        // Absolutely no idea why the client sets chunks-per-tick to very low values when sending this packet multiple times.
        // While testing the chunks-per-tick drop from around 5 to near zero.
        // self.send_packet(ChunkBatchFinishedPacket { batch_size });
        // queue.lead += 1;
    }

    fn keep_alive(&self) {
        self.send_packet(KeepAlivePacket { keep_alive_id: 0 });
    }

    // fn next_teleport_id(&self) -> i32 {
    //     self.teleport_id.fetch_add(1, Ordering::Release)
    // }
}

impl Tickable for Player {
    fn tick(self: &Arc<Self>) {
        let mut last_keep_alive = self.last_keep_alive.lock();
        if last_keep_alive.elapsed() > Duration::from_secs(20) {
            self.keep_alive();
            *last_keep_alive = Instant::now();
        }

        self.send_pending_chunks();
    }
}
