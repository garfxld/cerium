use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaintingVariant {
    pub asset_id: String,
    pub width: i32,
    pub height: i32,
}
