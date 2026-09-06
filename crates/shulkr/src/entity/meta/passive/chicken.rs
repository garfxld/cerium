use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::chicken::{SOUND_VARIANT, VARIANT},
    },
    registry::RegistryKey,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChickenVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub baby_asset_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChickenSoundVariant {
    adult_sounds: ChickenSoundSet,
    baby_sounds: ChickenSoundSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChickenSoundSet {
    ambient_sound: String,
    hurt_sound: String,
    death_sound: String,
    step_sound: String,
}

pub struct ChickenMeta {
    holder: MetadataHolder,
}

impl ChickenMeta {
    pub fn get_variant(&self) -> RegistryKey<ChickenVariant> {
        self.holder.get(VARIANT)
    }

    pub fn set_variant(&self, value: RegistryKey<ChickenVariant>) {
        self.holder.set(VARIANT, value);
    }

    pub fn get_sound_variant(&self) -> RegistryKey<ChickenSoundVariant> {
        self.holder.get(SOUND_VARIANT)
    }

    pub fn set_sound_variant(&self, value: RegistryKey<ChickenSoundVariant>) {
        self.holder.set(SOUND_VARIANT, value);
    }
}

impl MetaAccessor for ChickenMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
