use axum::{
    routing::{get, post},
    Extension, Json,
};
use http_problem::Result;

use crate::{
    domains::instance::{CreateInstanceData, Instance},
    handlers::instance::DynInstanceHandler,
};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
}

async fn list(
    Extension(instance_handler): Extension<DynInstanceHandler>,
) -> Result<Json<Vec<Instance>>> {
    instance_handler.list().await.map(Json)
}

async fn create(
    Json(data): Json<CreateInstanceData>,
    Extension(instance_handler): Extension<DynInstanceHandler>,
) -> Result<Json<Instance>> {
    instance_handler.create(data).await.map(Json)
}
