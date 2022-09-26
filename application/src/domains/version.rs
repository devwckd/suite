use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Version {
    pub id: Uuid,
    pub version: String,
    pub created_at: NaiveDateTime,

    pub blueprint_id: Uuid,
    pub env_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateVersionData {
    pub version: String,

    pub blueprint_id: Uuid,
    pub env_id: Uuid,
}
