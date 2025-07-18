use std::{collections::HashMap, sync::LazyLock};

use crate::registry::{
    biome::Biome, cat_variant::CatVariant, chicken_variant::ChickenVariant,
    cow_variant::CowVariant, damage_type::DamageType, dimension_type::DimensionType,
    frog_variant::FrogVariant, painting_variant::PaintingVariant, pig_variant::PigVariant,
    wolf_sound_variant::WolfSoundVariant, wolf_variant::WolfVariant,
};

#[derive(Debug, Clone)]
pub struct Registry<T>
where
    T: serde::de::DeserializeOwned,
{
    name: String,
    pub entries: HashMap<String, T>,
}

impl<T> Registry<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn new(name: String, data: String) -> Self {
        let entries: HashMap<String, T> = serde_json::from_str(&data).unwrap();
        Self { name, entries }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get(&self, key: String) -> Option<&T> {
        self.entries.get(&key)
    }

    pub fn get_id(&self, key: String) -> Option<usize> {
        self.entries.keys().position(|k| *k == key)
    }

}

pub struct Registries {
    pub biome: Registry<Biome>,
    pub cat_variant: Registry<CatVariant>,
    pub chicken_variant: Registry<ChickenVariant>,
    pub cow_variant: Registry<CowVariant>,
    pub damage_type: Registry<DamageType>,
    pub dimension_type: Registry<DimensionType>,
    pub frog_variant: Registry<FrogVariant>,
    pub painting_variant: Registry<PaintingVariant>,
    pub pig_variant: Registry<PigVariant>,
    pub wolf_sound_variant: Registry<WolfSoundVariant>,
    pub wolf_variant: Registry<WolfVariant>,
}

pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(|| {
    Registries::new()
});


impl Registries {

    pub fn new() -> Self {
        Self {
            biome: Registry::new(
                "minecraft:worldgen/biome".into(),
                include_str!("../../assets/biome.json").to_owned(),
            ),
            cat_variant: Registry::new(
                "minecraft:cat_variant".into(),
                include_str!("../../assets/cat_variant.json").to_owned(),
            ),
            chicken_variant: Registry::new(
                "minecraft:chicken_variant".into(),
                include_str!("../../assets/chicken_variant.json").to_owned(),
            ),
            cow_variant: Registry::new(
                "minecraft:cow_variant".into(),
                include_str!("../../assets/cow_variant.json").to_owned(),
            ),
            damage_type: Registry::new(
                "minecraft:damage_type".into(),
                include_str!("../../assets/damage_type.json").to_owned(),
            ),
            dimension_type: Registry::new(
                "minecraft:dimension_type".into(),
                include_str!("../../assets/dimension_type.json").to_owned(),
            ),
            frog_variant: Registry::new(
                "minecraft:frog_variant".into(),
                include_str!("../../assets/frog_variant.json").to_owned(),
            ),
            painting_variant: Registry::new(
                "minecraft:painting_variant".into(),
                include_str!("../../assets/painting_variant.json").to_owned(),
            ),
            pig_variant: Registry::new(
                "minecraft:pig_variant".into(),
                include_str!("../../assets/pig_variant.json").to_owned(),
            ),
            wolf_sound_variant: Registry::new(
                "minecraft:wolf_sound_variant".into(),
                include_str!("../../assets/wolf_sound_variant.json").to_owned(),
            ),
            wolf_variant: Registry::new(
                "minecraft:wolf_variant".into(),
                include_str!("../../assets/wolf_variant.json").to_owned(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::{cat_variant::CatVariant, registry::Registry};

    #[test]
    fn test_1() {
        let registry: Registry<CatVariant> = Registry::new(
            "".into(),
            include_str!("../../assets/cat_variant.json").to_owned(),
        );

        assert_eq!(
            registry.get("minecraft:black".to_owned()).unwrap().asset_id,
            "minecraft:entity/cat/black"
        )
    }
}
