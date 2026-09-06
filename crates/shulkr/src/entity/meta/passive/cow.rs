use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::cow::{SOUND_VARIANT, VARIANT},
    },
    registry::RegistryKey,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CowVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub baby_asset_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CowSoundVariant {
    ambient_sound: String,
    hurt_sound: String,
    death_sound: String,
    step_sound: String,
}

pub struct CowMeta {
    holder: MetadataHolder,
}

impl CowMeta {
    pub fn get_variant(&self) -> RegistryKey<CowVariant> {
        self.holder.get(VARIANT)
    }

    pub fn set_variant(&self, value: RegistryKey<CowVariant>) {
        self.holder.set(VARIANT, value);
    }

    pub fn get_sound_variant(&self) -> RegistryKey<CowSoundVariant> {
        self.holder.get(SOUND_VARIANT)
    }

    pub fn set_sound_variant(&self, value: RegistryKey<CowSoundVariant>) {
        self.holder.set(SOUND_VARIANT, value);
    }
}

impl MetaAccessor for CowMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
