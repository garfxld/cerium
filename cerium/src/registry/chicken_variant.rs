use cerium_nbt::{NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChickenVariant {
    pub asset_id: String,
}

impl ToNbt for ChickenVariant {
    fn to_nbt(self) -> cerium_nbt::Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("asset_id", NbtTag::String(self.asset_id));
        compound.into()
    }
}
