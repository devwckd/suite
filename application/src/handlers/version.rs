use std::sync::Arc;

use chrono::Utc;
use http_problem::{http::bad_request, Result};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domains::version::{CreateVersionData, Version},
    repositories::{
        blueprint::BlueprintRepository, env::EnvRepository, version::VersionRepository,
    },
};

pub type DynVersionHandler = Arc<dyn VersionHandler + Send + Sync>;

#[async_trait::async_trait]
pub trait VersionHandler {
    async fn list(&self) -> Result<Vec<Version>>;
    async fn create(&self, data: CreateVersionData) -> Result<Version>;
}

pub struct DefaultVersionHandler {
    pub blueprint_repository: Arc<dyn BlueprintRepository + Send + Sync>,
    pub env_repository: Arc<dyn EnvRepository + Send + Sync>,
    pub version_repository: Arc<dyn VersionRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl VersionHandler for DefaultVersionHandler {
    async fn list(&self) -> Result<Vec<Version>> {
        Ok(self.version_repository.get_all().await?)
    }

    async fn create(&self, data: CreateVersionData) -> Result<Version> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let version = Version {
            id: Uuid::new_v4(),
            version: data.version,
            created_at: Utc::now().naive_local(),
            blueprint_id: data.blueprint_id,
            env_id: data.env_id,
        };

        self.blueprint_repository
            .get_by_id(&version.blueprint_id)
            .await?;

        self.env_repository.get_by_id(&version.env_id).await?;

        self.version_repository.insert(&version).await?;

        log::info!("created version `{}`", &version.id);

        Ok(version)
    }
}
