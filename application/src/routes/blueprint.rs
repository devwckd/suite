use application::domains::blueprint::UpdateBlueprintData;
use axum::{
    extract::Path,
    routing::{get, patch, post},
    Extension, Json,
};
use http_problem::Result;
use uuid::Uuid;

use crate::{
    domains::blueprint::{Blueprint, CreateBlueprintData},
    handlers::blueprints::DynBlueprintHandler,
};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", patch(update))
}

async fn list(
    Extension(blueprint_handler): Extension<DynBlueprintHandler>,
) -> Result<Json<Vec<Blueprint>>> {
    blueprint_handler.list().await.map(Json)
}

async fn create(
    Json(data): Json<CreateBlueprintData>,
    Extension(blueprint_handler): Extension<DynBlueprintHandler>,
) -> Result<Json<Blueprint>> {
    blueprint_handler.create(data).await.map(Json)
}

async fn update(
    Path(blueprint_id): Path<Uuid>,
    Json(data): Json<UpdateBlueprintData>,
    Extension(blueprint_handler): Extension<DynBlueprintHandler>,
) -> Result<Json<Blueprint>> {
    blueprint_handler.update(blueprint_id, data).await.map(Json)
}
