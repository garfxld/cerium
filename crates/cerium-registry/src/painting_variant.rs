#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct PaintingVariant {
    pub asset_id: String,
    pub width: i32,
    pub height: i32,
}
