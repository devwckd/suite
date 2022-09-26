use axum::{
    routing::{get, post},
    Extension, Json,
};
use http_problem::Result;

use crate::{
    domains::version::{CreateVersionData, Version},
    handlers::version::DynVersionHandler,
};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
}

async fn list(
    Extension(version_handler): Extension<DynVersionHandler>,
) -> Result<Json<Vec<Version>>> {
    version_handler.list().await.map(Json)
}

async fn create(
    Json(data): Json<CreateVersionData>,
    Extension(version_handler): Extension<DynVersionHandler>,
) -> Result<Json<Version>> {
    version_handler.create(data).await.map(Json)
}
