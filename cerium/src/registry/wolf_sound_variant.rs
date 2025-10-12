use cerium_nbt::{Nbt, NbtCompound, ToNbt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfSoundVariant {
    pub hurt_sound: String,
    pub pant_sound: String,
    pub whine_sound: String,
    pub ambient_sound: String,
    pub death_sound: String,
    pub growl_sound: String,
}

impl ToNbt for WolfSoundVariant {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("hurt_sound", self.hurt_sound);
        compound.insert("pant_sound", self.pant_sound);
        compound.insert("whine_sound", self.whine_sound);
        compound.insert("ambient_sound", self.ambient_sound);
        compound.insert("death_sound", self.death_sound);
        compound.insert("growl_sound", self.growl_sound);
        compound.into()
    }
}
