use std::sync::Arc;

use cerium_world::World;

use crate::{
    entity::player::Player,
    event::{player::PlayerEvent, Event},
};

pub struct PlayerConfigEvent {
    pub(crate) player: Arc<Player>,
    pub(crate) world: Option<World>,
}

impl Event for PlayerConfigEvent {}

impl PlayerEvent for PlayerConfigEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}

impl PlayerConfigEvent {
    pub fn set_world(&mut self, world: World) {
        self.world = Some(world)
    }

    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }
}
