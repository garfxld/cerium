#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct WolfVariant {
    pub assets: WolfAssets,
}

#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct WolfAssets {
    wild: String,
    tame: String,
    angry: String,
}
