#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct Biome {
    has_precipitation: bool,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_modifier: Option<String>,
    downfall: f32,
    effects: BiomeEffects,
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
struct BiomeEffects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_color_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    particle: Option<Particle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mood_sound: Option<MoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additions_sound: Option<AdditionsSound>,
    //   #[serde(skip_serializing_if = "Option::is_none")]
    //   music: Option<Vec<DataPool<Music>>>,
    music_volume: f32,
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
struct Particle {
    options: ParticleOptions,
    probability: f32,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ParticleOptions {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<i32>,
}
use simdnbt::{
    ToNbtTag,
    owned::{NbtCompound, NbtTag},
};

impl ToNbtTag for ParticleOptions {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        let mut values = vec![("type".into(), NbtTag::String(self.r#type.clone().into()))];
        if let Some(value) = self.value {
            values.push(("value".into(), NbtTag::Int(value)));
        }

        NbtTag::Compound(NbtCompound::from_values(values))
    }
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
struct MoodSound {
    sound: String,
    tick_delay: i32,
    block_search_extent: i32,
    offset: f64,
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
struct AdditionsSound {
    sound: String,
    tick_chance: f64,
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    replace_current_music: bool,
}
