use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PigVariant {
    pub asset_id: String,
}

impl ToNbt for PigVariant {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("asset_id", self.asset_id);
        compound.into()
    }
}
