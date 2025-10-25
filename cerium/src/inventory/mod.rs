use std::{any::Any, sync::Arc};

use rustc_hash::FxHashMap;

mod inventory;
pub use inventory::*;

mod inventory_type;
pub use inventory_type::*;

mod player_inventory;
pub use player_inventory::PlayerInventory;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: FxHashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}
