use std::sync::Arc;

use chrono::Utc;
use http_problem::{http::bad_request, Result};
use uuid::Uuid;
use validator::Validate;

use crate::{
    actors::deploy_instance::DeployUnitHandle,
    domains::instance::{CreateInstanceData, Instance},
    repositories::{
        blueprint::BlueprintRepository, env::EnvRepository, instance::InstanceRepository,
        version::VersionRepository,
    },
};

pub type DynInstanceHandler = Arc<dyn InstanceHandler + Send + Sync>;

#[async_trait::async_trait]
pub trait InstanceHandler {
    async fn list(&self) -> Result<Vec<Instance>>;
    async fn create(&self, data: CreateInstanceData) -> Result<Instance>;
}

pub struct DefaultInstanceHandler {
    pub blueprint_repository: Arc<dyn BlueprintRepository + Send + Sync>,
    pub deploy_unit_handle: DeployUnitHandle,
    pub env_repository: Arc<dyn EnvRepository + Send + Sync>,
    pub instance_repository: Arc<dyn InstanceRepository + Send + Sync>,
    pub version_repository: Arc<dyn VersionRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl InstanceHandler for DefaultInstanceHandler {
    async fn list(&self) -> Result<Vec<Instance>> {
        self.instance_repository.get_all().await
    }

    async fn create(&self, data: CreateInstanceData) -> Result<Instance> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let blueprint = self
            .blueprint_repository
            .get_by_id(&data.blueprint_id)
            .await?;

        let env = self.env_repository.get_by_id(&data.env_id).await?;

        let version = self.version_repository.get_by_id(&data.version_id).await?;

        let instance = Instance {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            created_at: Utc::now().naive_local(),
            container: None,
            env_id: env.id,
            blueprint_id: blueprint.id,
            version_id: version.id,
        };

        self.instance_repository.upsert(&instance).await?;
        log::info!("created instance `{}`", &instance.name);

        self.deploy_unit_handle
            .deploy(&blueprint, &env, &instance, &version)
            .await;

        Ok(instance)
    }
}
