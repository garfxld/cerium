use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatVariant {
    pub asset_id: String,
}
