use rustc_hash::FxHashMap;

use crate::{
    entity::meta::{MetadataHolder, MetadataRef},
    util::EntityPose,
};

pub struct EntityMeta {
    pub holder: MetadataHolder,
}

impl EntityMeta {
    pub fn new() -> Self {
        Self {
            holder: MetadataHolder {
                entries: FxHashMap::default(),
            },
        }
    }
}

impl EntityMeta {
    pub fn is_on_fire(&self) -> bool {
        self.holder.get(MetadataRef::ON_FIRE).unwrap_or(false)
    }

    pub fn set_on_fire(&mut self, value: bool) {
        self.holder.set(MetadataRef::ON_FIRE, value);
    }

    pub fn is_sneaking(&self) -> bool {
        self.holder.get(MetadataRef::SNEAKING).unwrap_or(false)
    }

    pub fn set_sneaking(&mut self, value: bool) {
        self.holder.set(MetadataRef::SNEAKING, value);
    }

    pub fn is_sprinting(&self) -> bool {
        self.holder.get(MetadataRef::SPRINTING).unwrap_or(false)
    }

    pub fn set_sprinting(&mut self, value: bool) {
        self.holder.set(MetadataRef::SPRINTING, value);
    }

    pub fn set_pose(&mut self, pose: EntityPose) {
        self.holder.set(MetadataRef::POSE, pose);
    }

    pub fn get_pose(&self) -> EntityPose {
        self.holder
            .get(MetadataRef::POSE)
            .unwrap_or(EntityPose::Standing)
    }
}
