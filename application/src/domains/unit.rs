use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Unit {
    pub id: Uuid,
    pub container: String,
    pub image: String,
    pub env_variables: Vec<String>,
    pub exposed_port: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct CreateUnitData {
    pub image: String,
    pub env_variables: Vec<String>,
    pub exposed_port: Option<u64>,
}
