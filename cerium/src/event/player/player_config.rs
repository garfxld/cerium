use std::sync::Arc;

use crate::util::Position;
use crate::world::World;

use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

pub struct PlayerConfigEvent {
    pub(crate) player: Arc<Player>,
    pub(crate) world: Option<Arc<World>>,
    pub(crate) position: Option<Position>,
}

impl Event for PlayerConfigEvent {}

impl PlayerEvent for PlayerConfigEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}

impl PlayerConfigEvent {
    pub fn set_world(&mut self, world: Arc<World>) {
        self.world = Some(world);
    }

    pub fn get_world(&self) -> Option<Arc<World>> {
        self.world.clone()
    }

    pub fn set_position(&mut self, position: impl Into<Position>) {
        self.position = Some(position.into());
    }

    pub fn get_position(&self) -> Option<&Position> {
        self.position.as_ref()
    }
}
