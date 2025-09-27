use std::{any::Any, sync::Arc};

use rustc_hash::FxHashMap;

mod inventory;
pub use inventory::PlayerInventory;

pub mod item;
mod material;
pub use material::Material;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: FxHashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}
