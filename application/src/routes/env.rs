use application::domains::env::UpdateEnvData;
use axum::{
    extract::Path,
    routing::{get, patch, post},
    Extension, Json,
};
use http_problem::Result;
use uuid::Uuid;

use crate::{
    domains::env::{CreateEnvData, Env},
    handlers::env::DynEnvHandler,
};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", patch(update))
}

async fn list(Extension(env_handler): Extension<DynEnvHandler>) -> Result<Json<Vec<Env>>> {
    env_handler.list().await.map(Json)
}

async fn create(
    Json(data): Json<CreateEnvData>,
    Extension(env_handler): Extension<DynEnvHandler>,
) -> Result<Json<Env>> {
    env_handler.create(data).await.map(Json)
}

async fn update(
    Path(env_id): Path<Uuid>,
    Json(data): Json<UpdateEnvData>,
    Extension(env_handler): Extension<DynEnvHandler>,
) -> Result<Json<Env>> {
    env_handler.update(env_id, data).await.map(Json)
}
