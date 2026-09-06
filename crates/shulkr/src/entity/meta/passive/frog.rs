use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::frog::{TONGUE_TARGET, VARIANT},
    },
    registry::RegistryKey,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrogVariant {
    pub asset_id: String,
}

pub struct FrogMeta {
    holder: MetadataHolder,
}

impl FrogMeta {
    pub fn get_variant(&self) -> RegistryKey<FrogVariant> {
        self.holder.get(VARIANT)
    }

    pub fn set_variant(&self, value: RegistryKey<FrogVariant>) {
        self.holder.set(VARIANT, value);
    }

    pub fn get_tongue_target(&self) -> Option<i32> {
        self.holder.get(TONGUE_TARGET)
    }

    pub fn set_tongue_target(&self, value: Option<i32>) {
        self.holder.set(TONGUE_TARGET, value);
    }
}

impl MetaAccessor for FrogMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
