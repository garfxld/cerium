use std::sync::Arc;

use crate::{entity::Player, event::Event};

mod player_config;

pub use player_config::PlayerConfigEvent;

pub trait PlayerEvent
where
    Self: Event,
{
    fn get_player(&self) -> &Arc<Player>;
}
