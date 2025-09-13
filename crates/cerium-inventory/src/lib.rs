#![feature(const_type_id)]

use std::{any::Any, sync::Arc};

use rustc_hash::FxHashMap;

pub mod inventory;
pub mod item;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: FxHashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}
