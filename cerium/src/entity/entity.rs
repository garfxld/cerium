use std::sync::{
    Arc, Mutex,
    atomic::{AtomicI32, Ordering},
};
use uuid::Uuid;

use crate::{
    entity::{EntityType, Player},
    protocol::packet::{RemoveEntitiesPacket, SpawnEntityPacket},
    util::Position,
};

pub struct Entity {
    id: i32,
    uuid: Uuid,
    entity_type: EntityType,
    position: Mutex<Position>,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Arc<Self> {
        Arc::new(Self {
            id: Self::generate_id(),
            uuid: Uuid::new_v4(),
            entity_type,
            position: Mutex::new(Position::ZERO),
        })
    }

    /// Generates a new unique entity id.
    pub fn generate_id() -> i32 {
        static CURRENT_ID: AtomicI32 = AtomicI32::new(0);
        CURRENT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn position(&self) -> Position {
        self.position.lock().unwrap().clone()
    }

    pub fn set_position<P>(&self, position: P)
    where
        P: Into<Position>,
    {
        *self.position.lock().unwrap() = position.into();

        // todo: teleport
    }

    pub async fn show_for(&self, player: Arc<Player>) {
        player.send_packet::<SpawnEntityPacket>(self.into()).await;
    }

    pub async fn hide_for(&self, player: Arc<Player>) {
        player
            .send_packet(RemoveEntitiesPacket {
                entity_ids: vec![self.id()],
            })
            .await;
    }
}

impl Into<SpawnEntityPacket> for &Entity {
    fn into(self) -> SpawnEntityPacket {
        let position = self.position();

        SpawnEntityPacket {
            id: self.id(),
            uuid: self.uuid(),
            entity_type: self.entity_type() as i32,
            x: position.x(),
            y: position.y(),
            z: position.z(),
            pitch: position.pitch() as u8,
            yaw: position.yaw() as u8,
            head_yaw: position.yaw() as u8,
            data: 0,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        }
    }
}
