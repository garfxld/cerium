use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameProfile {
    #[serde(alias = "id")]
    pub uuid: Uuid,
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
