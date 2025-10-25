use std::sync::Arc;

use crate::{
    entity::Player,
    event::{inventory::InventoryEvent, player::PlayerEvent, Cancellable, Event},
    inventory::Inventory,
};

pub struct InventoryClickEvent {
    pub(crate) player: Arc<Player>,
    pub(crate) inventory: Arc<Inventory>,
    pub(crate) cancelled: bool,
}

impl Event for InventoryClickEvent {}

impl PlayerEvent for InventoryClickEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}

impl InventoryEvent for InventoryClickEvent {
    fn get_inventory(&self) -> &Arc<Inventory> {
        &self.inventory
    }
}

impl Cancellable for InventoryClickEvent {
    fn set_cancelled(&mut self, value: bool) {
        self.cancelled = value;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
