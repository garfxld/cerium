use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

include!("generated/blocks.rs");

impl Deref for Block {
    type Target = BlockState;

    fn deref(&self) -> &<Block as Deref>::Target {
        // None of these get calls should panic due to registry and auto-generate code accessing the same json file.
        let state_id = *REGISTRY.1.get(*self as usize).unwrap();
        REGISTRY.0.get(&state_id).unwrap()
    }
}

pub static REGISTRY: LazyLock<(HashMap<i32, BlockState>, Vec<i32>)> = LazyLock::new(|| {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/block.json")).unwrap();

    let mut states = HashMap::new();
    let mut to_state = vec![];

    for (_key, block) in entries {
        let id = block["id"].as_i64().unwrap() as i32;
        let block_entity: Option<BlockEntityInfo> =
            serde_json::from_value(block["blockEntity"].clone()).ok();

        to_state.push(block["defaultStateId"].as_i64().unwrap() as i32);

        for (_state_key, state) in block["states"].as_object().unwrap() {
            let state_id = state["stateId"].as_i64().unwrap() as usize;
            let state = BlockState {
                id,
                state_id: state_id as i32,
                block_entity: block_entity.clone(),
            };
            states.insert(state_id as i32, state);
        }
    }

    (states, to_state)
});

impl AsRef<BlockState> for BlockState {
    fn as_ref(&self) -> &BlockState {
        self
    }
}

impl AsRef<BlockState> for Block {
    fn as_ref(&self) -> &BlockState {
        &self.deref()
    }
}

#[derive(Debug, Clone)]
pub struct BlockState {
    id: i32,
    state_id: i32,
    block_entity: Option<BlockEntityInfo>,
}

impl BlockState {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn state_id(&self) -> i32 {
        self.state_id
    }

    pub fn block_entity(&self) -> Option<&BlockEntityInfo> {
        self.block_entity.as_ref()
    }

    pub fn from_id(id: i32) -> Option<&'static BlockState> {
        REGISTRY.0.get(&id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityInfo {
    pub namespace: String,
    pub id: i32,
}
