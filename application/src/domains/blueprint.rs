use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Blueprint {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateBlueprintData {
    pub name: String,
    pub image: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct UpdateBlueprintData {
    pub name: Option<String>,
    pub image: Option<String>,
}
