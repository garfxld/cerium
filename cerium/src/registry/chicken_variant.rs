use cerium_nbt::{NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChickenVariant {
    pub asset_id: String,
}
