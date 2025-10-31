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
