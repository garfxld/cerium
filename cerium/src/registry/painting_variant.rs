use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaintingVariant {
    pub asset_id: String,
    pub width: i32,
    pub height: i32,
}

impl ToNbt for PaintingVariant {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("asset_id", self.asset_id);
        compound.insert("width", self.width);
        compound.insert("height", self.height);
        compound.into()
    }
}
