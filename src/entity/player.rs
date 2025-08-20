use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::util::{Position, auth::GameProfile};
use crate::world::{World, chunk::Chunk};
use crate::{
    Server,
    protocol::{encode::Encode, packet::KeepAlivePacket},
};
use cerium_protocol::packet::{
    ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket,
    UnloadChunkPacket,
};

use tokio::time::{Interval, interval};
use uuid::Uuid;

use crate::{Tickable, network::client::ClientConnection};

pub struct ChunkQueue {
    queue: VecDeque<Chunk>,
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

    pub fn queue(&mut self, chunk: Chunk) {
        self.queue.push_back(chunk);
    }

    pub fn drain(&mut self) -> Box<[Chunk]> {
        let size = self.queue.len().min(self.chunks_per_tick as usize);
        let chunks: Vec<Chunk> = self.queue.drain(0..size).collect();
        chunks.into_boxed_slice()
    }
}

pub struct Player {
    connection: Arc<ClientConnection>,
    game_profile: GameProfile,
    world: Mutex<Option<Arc<World>>>,
    position: Mutex<Option<Position>>,
    pub(crate) chunk_queue: tokio::sync::Mutex<ChunkQueue>,
    last_keep_alive: tokio::sync::Mutex<Instant>,
    interval: tokio::sync::Mutex<Interval>,
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}

impl Player {
    pub async fn new(connection: Arc<ClientConnection>, _server: Arc<Server>) -> Self {
        let game_profile = connection.game_profile.lock().await.clone().unwrap();
        Self {
            connection,
            game_profile,
            world: Mutex::new(None),
            position: Mutex::new(None),
            chunk_queue: tokio::sync::Mutex::new(ChunkQueue::new()),
            last_keep_alive: tokio::sync::Mutex::new(Instant::now()),
            interval: tokio::sync::Mutex::new(interval(Duration::from_millis(50))),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn name(&self) -> String {
        self.game_profile.name.to_owned()
    }

    pub fn uuid(&self) -> Uuid {
        self.game_profile.uuid
    }

    #[inline]
    pub fn world(&self) -> Arc<World> {
        self.world.lock().unwrap().clone().unwrap()
    }

    #[inline]
    pub fn position(&self) -> Position {
        self.position.lock().unwrap().clone().unwrap()
    }

    pub(crate) fn set_world(&self, world: Arc<World>) {
        (*self.world.lock().unwrap()) = Some(world)
    }

    pub(crate) fn set_position(&self, position: Position) {
        (*self.position.lock().unwrap()) = Some(position)
    }

    pub async fn send_packet<P>(&self, packet_id: i32, packet: P)
    where
        P: Encode,
    {
        self.connection.send_packet(packet_id, packet).await;
    }

    pub(crate) async fn load_chunks(&self) {
        let chunk = Chunk::to_chunk_pos(self.position());
        let view_distance = 32;

        let world = self.world();
        for (cx, cz) in Chunk::chunks_in_range(chunk, view_distance) {
            let chunk = match world.get_chunk(cx, cz).await {
                Some(chunk) => chunk,
                None => world.load_chunk(cx, cz).await,
            };
            let chunk: tokio::sync::MutexGuard<'_, Chunk> = chunk.lock().await;
            self.send_chunk(chunk.clone()).await;
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

        let chunk = match world.get_chunk(cx, cz).await {
            Some(chunk) => chunk,
            None => world.load_chunk(cx, cz).await,
        };

        let chunk: tokio::sync::MutexGuard<'_, Chunk> = chunk.lock().await;
        self.send_chunk(chunk.clone()).await;
    }

    async fn unload_chunk(&self, cx: i32, cz: i32) {
        self.send_packet::<UnloadChunkPacket>(
            0x21,
            UnloadChunkPacket {
                chunk_x: cx,
                chunk_z: cz,
            },
        )
        .await;
    }

    async fn send_chunk(&self, chunk: Chunk) {
        self.chunk_queue.lock().await.queue(chunk.clone());
    }

    // works for now
    async fn flush_chunks(&self) {
        let mut queue = self.chunk_queue.lock().await;

        let chunks = queue.drain();
        let size = chunks.len();

        if size < 1 {
            return;
        }
        self.send_packet(0x0C, ChunkBatchStartPacket {}).await;

        for chunk in chunks {
            self.send_packet::<ChunkDataAndUpdateLightPacket>(0x27, (&chunk).into())
                .await;
        }
        self.send_packet(
            0x0B,
            ChunkBatchFinishedPacket {
                batch_size: size as i32,
            },
        )
        .await;
    }

    async fn keep_alive(&self) {
        self.send_packet(
            0x26,
            KeepAlivePacket {
                keep_alive_id: std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as i64,
            },
        )
        .await;
    }
}

impl Tickable for Player {
    async fn tick(self: &Arc<Self>) {
        self.interval.lock().await.tick().await;

        let mut last_keep_alive = self.last_keep_alive.lock().await;
        if last_keep_alive.elapsed() > Duration::from_secs(20) {
            self.keep_alive().await;
            *last_keep_alive = Instant::now();
        }

        self.flush_chunks().await;
    }
}
