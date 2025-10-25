use std::sync::Arc;

use crate::{
    entity::Player,
    event::{Event, inventory::InventoryEvent, player::PlayerEvent},
    inventory::Inventory,
};

pub struct InventoryCloseEvent {
    pub(crate) player: Arc<Player>,
    pub(crate) inventory: Arc<Inventory>,
}

impl Event for InventoryCloseEvent {}

impl PlayerEvent for InventoryCloseEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}

impl InventoryEvent for InventoryCloseEvent {
    fn get_inventory(&self) -> &Arc<Inventory> {
        &self.inventory
    }
}
