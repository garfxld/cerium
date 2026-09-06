use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::{
            tamable_animal::{IS_SITTING, IS_TAMED, OWNER},
            wolf::{ANGER_TIME, COLLAR_COLOR, IS_BEGGING, SOUND_VARIANT, VARIANT},
        },
    },
    registry::RegistryKey,
    util::DyeColor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    pub assets: WolfAssets,
    pub baby_assets: WolfAssets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfAssets {
    wild: String,
    tame: String,
    angry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfSoundVariant {
    pub adult_sounds: WolfSounds,
    pub baby_sounds: WolfSounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfSounds {
    pub hurt_sound: String,
    pub pant_sound: String,
    pub whine_sound: String,
    pub ambient_sound: String,
    pub death_sound: String,
    pub growl_sound: String,
    pub step_sound: String,
}

pub struct WolfMeta {
    holder: MetadataHolder,
}

impl WolfMeta {
    pub fn is_sitting(&self) -> bool {
        self.holder.get(IS_SITTING)
    }

    pub fn set_sitting(&self, value: bool) {
        self.holder.set(IS_SITTING, value);
    }

    pub fn is_tamed(&self) -> bool {
        self.holder.get(IS_TAMED)
    }

    pub fn set_tamed(&self, value: bool) {
        self.holder.set(IS_TAMED, value);
    }

    pub fn get_owner(&self) -> Option<Uuid> {
        self.holder.get(OWNER)
    }

    pub fn set_owner(&self, value: Option<Uuid>) {
        self.holder.set(OWNER, value);
    }

    pub fn is_begging(&self) -> bool {
        self.holder.get(IS_BEGGING)
    }

    pub fn set_begging(&self, value: bool) {
        self.holder.set(IS_BEGGING, value);
    }

    pub fn get_collar_color(&self) -> DyeColor {
        DyeColor::try_from(self.holder.get(COLLAR_COLOR)).unwrap_or(DyeColor::Red)
    }

    pub fn set_collar_color(&self, value: DyeColor) {
        self.holder.set(COLLAR_COLOR, value as i32);
    }

    pub fn get_anger_time(&self) -> i64 {
        self.holder.get(ANGER_TIME)
    }

    pub fn set_anger_time(&self, value: i64) {
        self.holder.set(ANGER_TIME, value);
    }

    pub fn get_variant(&self) -> RegistryKey<WolfVariant> {
        self.holder.get(VARIANT)
    }

    pub fn set_variant(&self, value: RegistryKey<WolfVariant>) {
        self.holder.set(VARIANT, value);
    }

    pub fn get_sound_variant(&self) -> RegistryKey<WolfSoundVariant> {
        self.holder.get(SOUND_VARIANT)
    }

    pub fn set_sound_variant(&self, value: RegistryKey<WolfSoundVariant>) {
        self.holder.set(SOUND_VARIANT, value);
    }
}

impl MetaAccessor for WolfMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
