use cerium_nbt::{Nbt, NbtCompound, NbtTag, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    pub assets: WolfAssets,
}

impl ToNbt for WolfVariant {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();

        let mut assets = NbtCompound::new();
        assets.insert("wild", self.assets.wild);
        assets.insert("tame", self.assets.tame);
        assets.insert("angry", self.assets.angry);

        compound.insert("assets", assets);
        compound.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfAssets {
    wild: String,
    tame: String,
    angry: String,
}
