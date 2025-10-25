use std::sync::Arc;

use crate::{event::player::PlayerEvent, inventory::Inventory};

mod click;
pub use click::InventoryClickEvent;

mod open;
pub use open::InventoryOpenEvent;

mod close;
pub use close::InventoryCloseEvent;

pub trait InventoryEvent: PlayerEvent {
    fn get_inventory(&self) -> &Arc<Inventory>;
}
