use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Env {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateEnvData {
    pub name: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct UpdateEnvData {
    pub name: Option<String>,
}
