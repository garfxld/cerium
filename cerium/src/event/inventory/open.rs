use crate::{
    entity::Player,
    event::{Cancellable, Event, inventory::InventoryEvent, player::PlayerEvent},
    inventory::Inventory,
};

pub struct InventoryOpenEvent {
    pub(crate) player: Player,
    pub(crate) inventory: Inventory,
    pub(crate) cancelled: bool,
}

impl Event for InventoryOpenEvent {}

impl PlayerEvent for InventoryOpenEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl InventoryEvent for InventoryOpenEvent {
    fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }
}

impl Cancellable for InventoryOpenEvent {
    fn set_cancelled(&mut self, value: bool) {
        self.cancelled = value;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
