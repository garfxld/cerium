use simdnbt::{
    ToNbtTag,
    owned::{NbtCompound, NbtTag},
};

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct DimensionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub monster_spawn_block_light_limit: i32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Int(i32),
    Tagged(MonsterSpawnLightLevelTagged),
}

impl ToNbtTag for MonsterSpawnLightLevel {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        match self {
            Self::Int(v) => NbtTag::Int(v),
            Self::Tagged(v) => v.to_nbt_tag(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MonsterSpawnLightLevelTagged {
    min_inclusive: i32,
    max_inclusive: i32,
    #[serde(rename = "type")]
    r#type: String,
}

impl ToNbtTag for MonsterSpawnLightLevelTagged {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        let values = vec![
            ("min_inclusive".into(), NbtTag::Int(self.min_inclusive)),
            ("max_inclusive".into(), NbtTag::Int(self.max_inclusive)),
            ("type".into(), NbtTag::String(self.r#type.clone().into())),
        ];
        NbtTag::Compound(NbtCompound::from_values(values))
    }
}
