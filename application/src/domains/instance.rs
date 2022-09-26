use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,

    pub container: Option<String>,

    pub blueprint_id: Uuid,
    pub env_id: Uuid,
    pub version_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateInstanceData {
    pub blueprint_id: Uuid,
    pub env_id: Uuid,
    pub version_id: Uuid,
}
