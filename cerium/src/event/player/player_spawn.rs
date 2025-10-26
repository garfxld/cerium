use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

pub struct PlayerSpawnEvent {
    pub(crate) player: Player,
}

impl Event for PlayerSpawnEvent {}

impl PlayerEvent for PlayerSpawnEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
