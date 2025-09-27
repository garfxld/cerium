#![allow(unused_imports)]

use crate::util::Identifier;
use indexmap::{IndexMap, map::Iter};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::LazyLock,
};

mod biome;
mod cat_variant;
mod chicken_variant;
mod cow_variant;
mod damage_type;
mod dimension_type;
mod frog_variant;
mod generated;
mod painting_variant;
mod pig_variant;
mod wolf_sound_variant;
mod wolf_variant;

pub use biome::*;
pub use cat_variant::*;
pub use chicken_variant::*;
pub use cow_variant::*;
pub use damage_type::*;
pub use dimension_type::*;
pub use dimension_type::*;
pub use frog_variant::*;
pub use generated::*;
pub use painting_variant::*;
pub use pig_variant::*;
pub use wolf_sound_variant::*;
pub use wolf_variant::*;

#[derive(Debug)]
pub struct DynamicRegistry<T>
where
    T: DeserializeOwned,
{
    registry_id: Identifier,
    entries: IndexMap<RegistryKey<T>, T>,
}

impl<T> DynamicRegistry<T>
where
    T: DeserializeOwned,
    RegistryKey<T>: Eq + Hash,
{
    pub fn create(name: String, data: String) -> Self {
        let mut entries: IndexMap<String, T> = serde_json::from_str(&data).unwrap();

        let mut this = Self {
            registry_id: Identifier::of(name),
            entries: IndexMap::new(),
        };

        let key = "plains";
        if let Some(value) = entries.swap_remove(key) {
            let old_entries = std::mem::take(&mut entries);
            entries.insert(key.to_string(), value);
            entries.extend(old_entries);
        }

        for (key, value) in entries {
            this.register(RegistryKey::of(key), value);
        }

        this
    }

    pub fn registry_id(&self) -> &Identifier {
        &self.registry_id
    }

    pub fn register(&mut self, key: RegistryKey<T>, object: T) {
        self.entries.insert(key, object);
    }

    pub fn get(&self, key: &RegistryKey<T>) -> Option<&T> {
        self.entries.get(key)
    }

    pub fn get_id(&self, key: &RegistryKey<T>) -> Option<usize> {
        self.entries.keys().position(|k| k == key)
    }

    pub fn entries(&self) -> &IndexMap<RegistryKey<T>, T> {
        &self.entries
    }
}

pub struct Registries {
    pub biome: DynamicRegistry<Biome>,
    pub cat_variant: DynamicRegistry<CatVariant>,
    pub chicken_variant: DynamicRegistry<ChickenVariant>,
    pub cow_variant: DynamicRegistry<CowVariant>,
    pub damage_type: DynamicRegistry<DamageType>,
    pub dimension_type: DynamicRegistry<DimensionType>,
    pub frog_variant: DynamicRegistry<FrogVariant>,
    pub painting_variant: DynamicRegistry<PaintingVariant>,
    pub pig_variant: DynamicRegistry<PigVariant>,
    pub wolf_sound_variant: DynamicRegistry<WolfSoundVariant>,
    pub wolf_variant: DynamicRegistry<WolfVariant>,
}

pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(|| Registries::new());

impl Registries {
    pub fn new() -> Self {
        Self {
            biome: DynamicRegistry::create(
                "minecraft:worldgen/biome".into(),
                include_str!("../../data/biome.json").to_owned(),
            ),
            cat_variant: DynamicRegistry::create(
                "minecraft:cat_variant".into(),
                include_str!("../../data/cat_variant.json").to_owned(),
            ),
            chicken_variant: DynamicRegistry::create(
                "minecraft:chicken_variant".into(),
                include_str!("../../data/chicken_variant.json").to_owned(),
            ),
            cow_variant: DynamicRegistry::create(
                "minecraft:cow_variant".into(),
                include_str!("../../data/cow_variant.json").to_owned(),
            ),
            damage_type: DynamicRegistry::create(
                "minecraft:damage_type".into(),
                include_str!("../../data/damage_type.json").to_owned(),
            ),
            dimension_type: DynamicRegistry::create(
                "minecraft:dimension_type".into(),
                include_str!("../../data/dimension_type.json").to_owned(),
            ),
            frog_variant: DynamicRegistry::create(
                "minecraft:frog_variant".into(),
                include_str!("../../data/frog_variant.json").to_owned(),
            ),
            painting_variant: DynamicRegistry::create(
                "minecraft:painting_variant".into(),
                include_str!("../../data/painting_variant.json").to_owned(),
            ),
            pig_variant: DynamicRegistry::create(
                "minecraft:pig_variant".into(),
                include_str!("../../data/pig_variant.json").to_owned(),
            ),
            wolf_sound_variant: DynamicRegistry::create(
                "minecraft:wolf_sound_variant".into(),
                include_str!("../../data/wolf_sound_variant.json").to_owned(),
            ),
            wolf_variant: DynamicRegistry::create(
                "minecraft:wolf_variant".into(),
                include_str!("../../data/wolf_variant.json").to_owned(),
            ),
        }
    }
}

#[derive(Debug)]
pub struct RegistryKey<T> {
    key: Identifier,
    _phantom: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    pub fn of<S>(key: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            key: Identifier::of(key),
            _phantom: PhantomData,
        }
    }

    pub fn as_key(&self) -> &Identifier {
        &self.key
    }

    pub fn to_key(self) -> Identifier {
        self.key
    }
}

impl<T> PartialEq for RegistryKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for RegistryKey<T> {}

impl<T> Hash for RegistryKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
