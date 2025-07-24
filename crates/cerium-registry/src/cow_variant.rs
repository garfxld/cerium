#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct CowVariant {
    pub asset_id: String,
}
