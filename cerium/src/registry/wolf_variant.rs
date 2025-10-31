use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    pub assets: WolfAssets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfAssets {
    wild: String,
    tame: String,
    angry: String,
}
