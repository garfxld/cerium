#[derive(Debug, Clone, serde::Deserialize, simdnbt::Serialize)]
pub struct DamageType {
    exhaustion: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    death_message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effects: Option<String>,
    message_id: String,
    scaling: String,
}
