use crate::{
    entity::Player,
    event::{Cancellable, Event, inventory::InventoryEvent, player::PlayerEvent},
    inventory::Inventory,
};

pub struct InventoryClickEvent {
    pub(crate) player: Player,
    pub(crate) inventory: Inventory,
    pub(crate) cancelled: bool,
}

impl Event for InventoryClickEvent {}

impl PlayerEvent for InventoryClickEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl InventoryEvent for InventoryClickEvent {
    fn get_inventory(&self) -> &Inventory {
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
