use std::sync::Arc;

use application::domains::blueprint::UpdateBlueprintData;
use chrono::Utc;
use http_problem::{http::bad_request, Result};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domains::blueprint::{Blueprint, CreateBlueprintData},
    repositories::blueprint::BlueprintRepository,
};

pub type DynBlueprintHandler = Arc<dyn BlueprintHandler + Send + Sync>;

#[async_trait::async_trait]
pub trait BlueprintHandler {
    async fn list(&self) -> Result<Vec<Blueprint>>;
    async fn create(&self, data: CreateBlueprintData) -> Result<Blueprint>;
    async fn update(&self, id: Uuid, data: UpdateBlueprintData) -> Result<Blueprint>;
}

pub struct DefaultBlueprintHandler {
    pub blueprint_repository: Arc<dyn BlueprintRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl BlueprintHandler for DefaultBlueprintHandler {
    async fn list(&self) -> Result<Vec<Blueprint>> {
        Ok(self.blueprint_repository.get_all().await?)
    }

    async fn create(&self, data: CreateBlueprintData) -> Result<Blueprint> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let blueprint = Blueprint {
            id: Uuid::new_v4(),
            name: data.name,
            image: data.image,
            created_at: Utc::now().naive_local(),
        };
        self.blueprint_repository.insert(&blueprint).await?;

        log::info!("created blueprint `{}`", &blueprint.name);

        Ok(blueprint)
    }

    async fn update(&self, id: Uuid, data: UpdateBlueprintData) -> Result<Blueprint> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let mut blueprint = self.blueprint_repository.get_by_id(&id).await?;

        if let Some(name) = data.name {
            blueprint.name = name;
        }

        if let Some(image) = data.image {
            blueprint.image = image;
        }

        self.blueprint_repository.upsert(&blueprint).await?;

        log::info!("edited blueprint `{}`", &blueprint.name);

        Ok(blueprint)
    }
}
