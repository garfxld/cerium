use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, Ordering},
    },
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::{
    Server,
    auth::GameProfile,
    entity::{Entity, EntityType, GameMode, entity::EntityLike},
    event::{Cancellable, inventory::InventoryOpenEvent},
    inventory::{Inventory, PlayerInventory},
    network::client::Connection,
    protocol::packet::{
        ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket, EntityPositionRotationPacket,
        EntityRotationPacket, GameEventPacket, Packet, PlayerAbilities, PlayerAction, PlayerEntry,
        PlayerInfoFlags, PlayerInfoRemovePacket, PlayerInfoUpdatePacket, ServerPacket,
        SetCenterChunkPacket, SetHeadRotationPacket, SyncPlayerPositionPacket,
        SystemChatMessagePacket, UnloadChunkPacket,
        server::{PlayerAbilitiesPacket, play::KeepAlivePacket},
    },
    text::Component,
    tickable::Tickable,
    util::{EntityPose, Position, TeleportFlags, Viewable},
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
    last_keep_alive: Mutex<Instant>,
    inventory: Arc<PlayerInventory>,
    game_mode: Mutex<GameMode>,
    pub(crate) chunk_queue: Mutex<ChunkQueue>,
    teleport_id: AtomicI32,
    flying: AtomicBool,
    allow_flying: AtomicBool,
    invurnable: AtomicBool,
    flying_speed: Mutex<f32>,
    fov_modifier: Mutex<f32>,
    open_inventory: Mutex<Option<Arc<Inventory>>>,
    server: Arc<Server>,
}

impl Player {
    pub fn new(connection: Arc<Connection>, server: Arc<Server>) -> Self {
        let game_profile = connection.game_profile.lock().clone().unwrap();
        Self {
            connection,
            game_profile: game_profile.clone(),
            entity: Entity::new_with_uuid(EntityType::Player, game_profile.uuid),
            world: Mutex::new(None),
            last_keep_alive: Mutex::new(Instant::now()),
            inventory: Arc::new(PlayerInventory::new()),
            game_mode: Mutex::new(GameMode::Survival),
            chunk_queue: Mutex::new(ChunkQueue::new()),
            teleport_id: AtomicI32::default(),
            flying: AtomicBool::default(),
            allow_flying: AtomicBool::default(),
            invurnable: AtomicBool::default(),
            flying_speed: Mutex::new(0.05),
            fov_modifier: Mutex::new(0.1),
            open_inventory: Mutex::new(None),
            server,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn name(&self) -> &String {
        &self.game_profile.name
    }

    pub fn game_mode(&self) -> GameMode {
        *self.game_mode.lock()
    }

    pub(crate) fn set_position(&self, position: Position) {
        self.entity.set_position(position);
    }

    pub fn set_game_mode(&self, game_mode: GameMode) {
        {
            *self.game_mode.lock() = game_mode;
        }

        self.send_packet(GameEventPacket {
            event: 3,
            value: game_mode as i32 as f32,
        });

        self.set_allow_flying(game_mode == GameMode::Creative || game_mode == GameMode::Spectator);
        if game_mode != GameMode::Creative && game_mode != GameMode::Spectator {
            self.set_flying(false);
        }

        self.refresh_abilities();
    }

    pub fn send_message(&self, message: impl Into<Component>) {
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

    fn keep_alive(&self) {
        self.send_packet(KeepAlivePacket { keep_alive_id: 0 });
    }

    pub(crate) fn add_to_list_packet(&self) -> PlayerInfoUpdatePacket {
        PlayerInfoUpdatePacket {
            actions: (PlayerInfoFlags::ADD_PLAYER | PlayerInfoFlags::UPDATE_LISTED).bits(),
            players: vec![PlayerEntry {
                uuid: self.uuid(),
                player_actions: vec![
                    PlayerAction::AddPlayer {
                        name: self.game_profile.name.clone(),
                        properties: self.game_profile.properties.clone(),
                    },
                    PlayerAction::UpdateListed { listed: true },
                ],
            }],
        }
    }

    pub fn server(&self) -> &Arc<Server> {
        &self.server
    }

    // ===== Inventory ======

    /// Returns the player's inventory.
    ///
    /// Note: this is not the open inventory. Use [`Player#get_open_inventory()`] instead.
    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.inventory
    }

    /// Opens an [`Inventory`] for a player.
    pub fn open_inventory(self: Arc<Self>, inventory: Arc<Inventory>) {
        let mut event = InventoryOpenEvent {
            player: self.clone(),
            inventory: inventory.clone(),
            cancelled: false,
        };
        self.server.events().fire(&mut event);

        if event.is_cancelled() {
            return;
        }

        if let Some(inventory) = self.get_open_inventory() {
            inventory.remove_viewer(self.clone());
        }

        inventory.add_viewer(self.clone());
        *self.open_inventory.lock() = Some(inventory);
    }

    /// Closes the opened inventory if it is open.
    pub fn close_inventory(self: Arc<Self>) {
        let inventory = self.open_inventory.lock().clone();
        if let Some(inventory) = inventory {
            inventory.remove_viewer(self);
        }
    }

    /// Returns the open inventory.
    pub fn get_open_inventory(&self) -> Option<Arc<Inventory>> {
        self.open_inventory.lock().clone()
    }

    // ===== World ======

    pub(crate) fn update_chunks(&self, new_chunk: (i32, i32), old_chunk: (i32, i32)) {
        let view_distance = 8;

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

    pub(crate) fn set_world(&self, world: Arc<World>) {
        (*self.world.lock()) = Some(world)
    }

    // ===== Position & Movement ======

    pub fn refresh_position(&self, new_position: Position) {
        let old_position = self.position();

        self.set_position(new_position);
        self.set_head_roation(new_position.yaw());

        let old_chunk = Chunk::to_chunk_pos(old_position);
        let new_chunk = Chunk::to_chunk_pos(new_position);

        if old_chunk != new_chunk {
            self.send_packet(SetCenterChunkPacket {
                chunk_x: new_chunk.0,
                chunk_z: new_chunk.1,
            });
            self.update_chunks(new_chunk, old_chunk);
        }

        let head_rotation = new_position.yaw();

        let distance_x = (new_position.x() - old_position.x()).abs();
        let distance_y = (new_position.y() - old_position.y()).abs();
        let distance_z = (new_position.z() - old_position.z()).abs();

        let position_changed = (distance_x + distance_y + distance_z) > 0.;
        let rotation_changed = (new_position.yaw() != old_position.yaw())
            || (new_position.pitch() != old_position.pitch());

        let on_ground = self.is_on_ground();
        match () {
            _ if distance_x > 8. || distance_y > 8. || distance_z > 8. => {
                log::warn!("todo: teleport player because he moved more than 8 blocks.")
            }
            _ if position_changed && rotation_changed => {
                self.send_packet_to_viewers(EntityPositionRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
                self.send_packet_to_viewers(SetHeadRotationPacket::new(self.id(), head_rotation));
            }
            _ if position_changed => {
                self.send_packet_to_viewers(EntityPositionRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
            }
            _ if rotation_changed => {
                self.send_packet_to_viewers(EntityRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
                self.send_packet_to_viewers(SetHeadRotationPacket::new(self.id(), head_rotation));
            }
            _ => {
                log::error!("Entered unreachable code.");
                self.connection.close();
            }
        }
    }

    pub fn synchronize_position(
        &self,
        position: Position,
        velocity: Position,
        flags: TeleportFlags,
    ) {
        let teleport_id = self.next_teleport_id();
        self.send_packet(SyncPlayerPositionPacket {
            teleport_id,
            position,
            velocity_x: velocity.x(),
            velocity_y: velocity.y(),
            velocity_z: velocity.z(),
            yaw: position.yaw(),
            pitch: position.pitch(),
            flags,
        });
    }

    pub fn head_roation(&self) -> f32 {
        self.entity.head_roation()
    }

    pub fn set_head_roation(&self, value: f32) {
        self.entity.set_head_roation(value);
    }

    pub fn is_on_ground(&self) -> bool {
        self.entity.is_on_ground()
    }

    pub fn refresh_on_ground(&self, value: bool) {
        self.entity.refresh_on_ground(value);
    }

    fn next_teleport_id(&self) -> i32 {
        self.teleport_id.fetch_add(1, Ordering::Release)
    }

    // ===== Abilities ======

    /// Returns if the player is invurnable.
    pub fn invurnable(&self) -> bool {
        self.invurnable.load(Ordering::Acquire)
    }

    pub fn set_invurnable(&self, value: bool) {
        self.invurnable.store(value, Ordering::Release);
    }

    /// Returns the flying speed of the player.
    pub fn flying_speed(&self) -> f32 {
        *self.flying_speed.lock()
    }

    pub fn set_flying_speed(&self, value: f32) {
        {
            *self.flying_speed.lock() = value;
        }
        self.refresh_abilities();
    }

    /// Returns the fov modifier of the player.
    pub fn fov_modifier(&self) -> f32 {
        *self.fov_modifier.lock()
    }

    pub fn set_fov_modifier(&self, value: f32) {
        {
            *self.fov_modifier.lock() = value;
        }
        self.refresh_abilities();
    }

    /// Returns if the player is allowed to fly.
    pub fn allow_flying(&self) -> bool {
        self.allow_flying.load(Ordering::Acquire)
    }

    pub fn set_allow_flying(&self, value: bool) {
        self.allow_flying.store(value, Ordering::Release);
    }

    /// Returns if the player is currently flying.
    pub fn flying(&self) -> bool {
        self.flying.load(Ordering::Acquire)
    }

    pub fn set_flying(&self, value: bool) {
        if self.flying() != value {
            let pose = self.entity.pose();
            match () {
                _ if self.is_sneaking() && pose == EntityPose::Standing => {
                    self.set_pose(EntityPose::Sneaking);
                }
                _ if pose == EntityPose::Sneaking => {
                    self.set_pose(EntityPose::Standing);
                }
                _ => {}
            }
        }

        self.flying.store(value, Ordering::Release);
    }

    pub fn refresh_abilities(&self) {
        let mut flags = PlayerAbilities::empty();
        if self.invurnable() {
            flags |= PlayerAbilities::INVURNABLE;
        }
        if self.flying() {
            flags |= PlayerAbilities::FLYING;
        }
        if self.allow_flying() {
            flags |= PlayerAbilities::ALLOW_FLYING;
        }

        self.send_packet(PlayerAbilitiesPacket {
            flags,
            flying_speed: *self.flying_speed.lock(),
            fov_modifier: *self.fov_modifier.lock(),
        });
    }

    pub fn set_pose(&self, pose: EntityPose) {
        self.entity.set_pose(pose);
        self.send_packet(self.entity.metadata_packet());
    }

    /// Returns if the player is sprinting.
    pub fn is_sprinting(&self) -> bool {
        self.entity.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.entity.set_sprinting(value);
        self.send_packet(self.entity.metadata_packet());
    }

    /// Returns if the player is sneaking.
    pub fn is_sneaking(&self) -> bool {
        self.entity.is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        self.entity.set_sneaking(value);
        self.send_packet(self.entity.metadata_packet());
    }

    pub fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }
    }
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

impl Viewable for Player {
    fn add_viewer(&self, player: Arc<Player>) {
        player.send_packet(self.add_to_list_packet());

        self.entity.add_viewer(player);
    }

    fn remove_viewer(&self, player: Arc<Player>) {
        player.send_packet(PlayerInfoRemovePacket {
            uuids: vec![self.uuid()],
        });
        self.entity.remove_viewer(player);
    }

    fn viewers(&self) -> Vec<Arc<Player>> {
        self.entity.viewers()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }
}

impl EntityLike for Player {
    fn id(&self) -> i32 {
        self.entity.id()
    }

    fn uuid(&self) -> Uuid {
        self.entity.uuid()
    }

    fn r#type(&self) -> EntityType {
        self.entity.r#type()
    }

    fn world(&self) -> Arc<World> {
        self.world.lock().clone().unwrap()
    }

    fn position(&self) -> Position {
        self.entity.position()
    }
}
