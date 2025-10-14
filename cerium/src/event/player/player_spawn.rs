use std::sync::Arc;

use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

pub struct PlayerSpawnEvent {
    pub(crate) player: Arc<Player>,
}

impl Event for PlayerSpawnEvent {}

impl PlayerEvent for PlayerSpawnEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}
