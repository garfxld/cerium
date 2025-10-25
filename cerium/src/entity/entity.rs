use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, Ordering},
};
use uuid::Uuid;

use crate::{
    entity::{EntityType, Player, meta::entity::EntityMeta},
    protocol::packet::{RemoveEntitiesPacket, SetEntityMetadataPacket, SpawnEntityPacket},
    util::{EntityPose, Position, Viewable},
    world::World,
};

pub struct Entity {
    id: i32,
    uuid: Uuid,
    entity_type: EntityType,
    position: Mutex<Position>,
    head_roation: Mutex<f32>,
    meta: Mutex<EntityMeta>,
    on_ground: AtomicBool,
    viewers: Mutex<Vec<Arc<Player>>>,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Arc<Self> {
        Self::new_with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn new_with_uuid(entity_type: EntityType, uuid: Uuid) -> Arc<Self> {
        Arc::new(Self {
            id: Self::generate_id(),
            uuid,
            entity_type,
            position: Mutex::new(Position::ZERO),
            head_roation: Mutex::new(0.),
            meta: Mutex::new(EntityMeta::new()),
            on_ground: AtomicBool::default(),
            viewers: Mutex::new(vec![]),
        })
    }

    fn generate_id() -> i32 {
        static CURRENT_ID: AtomicI32 = AtomicI32::new(1);
        CURRENT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn head_roation(&self) -> f32 {
        *self.head_roation.lock()
    }

    pub fn set_head_roation(&self, value: f32) {
        *self.head_roation.lock() = value;
    }

    pub fn set_pose(&self, pose: EntityPose) {
        {
            let mut meta = self.meta.lock();
            meta.set_pose(pose);
        }
        self.refresh_meta();
    }

    pub fn set_position<P>(&self, position: P)
    where
        P: Into<Position>,
    {
        *self.position.lock() = position.into();

        // todo: teleport
    }

    pub fn set_on_fire(&self, value: bool) {
        {
            let mut meta = self.meta.lock();
            meta.set_on_fire(value);
        }
        self.refresh_meta();
    }

    pub fn is_sneaking(&self) -> bool {
        self.meta.lock().is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        {
            let mut meta = self.meta.lock();
            meta.set_sneaking(value);
        }
        self.refresh_meta();
        self.set_pose(if value {
            EntityPose::Sneaking
        } else {
            EntityPose::Standing
        });
    }

    fn refresh_meta(&self) {
        self.send_packet_to_viewers(self.metadata_packet());
    }

    pub fn is_sprinting(&self) -> bool {
        let meta = self.meta.lock();
        meta.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        {
            let mut meta = self.meta.lock();
            meta.set_sprinting(value);
        }
        self.refresh_meta();
    }

    pub(crate) fn metadata_packet(&self) -> SetEntityMetadataPacket {
        SetEntityMetadataPacket {
            entity_id: self.id(),
            entries: self.meta.lock().holder.entries.clone(),
        }
    }

    pub fn pose(&self) -> EntityPose {
        self.meta.lock().get_pose()
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_ground.load(Ordering::Acquire)
    }

    pub fn refresh_on_ground(&self, value: bool) {
        self.on_ground.store(value, Ordering::Release);
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        let position = self.position();

        SpawnEntityPacket {
            id: self.id(),
            uuid: self.uuid(),
            entity_type: self.r#type() as i32,
            position,
            head_yaw: position.yaw(),
            data: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        }
    }

    pub fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }
    }
}

impl Viewable for Entity {
    fn add_viewer(&self, player: Arc<Player>) {
        self.viewers.lock().push(player.clone());

        player.send_packet(self.spawn_packet());
        player.send_packet(self.metadata_packet());
    }

    fn remove_viewer(&self, player: Arc<Player>) {
        self.viewers.lock().retain(|other| *other != player);

        player.send_packet(RemoveEntitiesPacket {
            entity_ids: vec![self.id()],
        });
    }

    fn viewers(&self) -> Vec<Arc<Player>> {
        (*self.viewers.lock()).clone()
    }
}

pub trait EntityLike {
    fn id(&self) -> i32;
    fn uuid(&self) -> Uuid;
    fn r#type(&self) -> EntityType;
    fn world(&self) -> Arc<World>;
    fn position(&self) -> Position;
}

impl EntityLike for Entity {
    fn id(&self) -> i32 {
        self.id
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn r#type(&self) -> EntityType {
        self.entity_type
    }

    fn world(&self) -> Arc<World> {
        todo!()
    }

    fn position(&self) -> Position {
        *self.position.lock()
    }
}
