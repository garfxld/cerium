use crate::{
    entity::Player,
    event::{Event, inventory::InventoryEvent, player::PlayerEvent},
    inventory::Inventory,
};

pub struct InventoryCloseEvent {
    pub(crate) player: Player,
    pub(crate) inventory: Inventory,
}

impl Event for InventoryCloseEvent {}

impl PlayerEvent for InventoryCloseEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl InventoryEvent for InventoryCloseEvent {
    fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }
}
