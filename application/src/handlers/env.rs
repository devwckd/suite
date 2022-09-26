use std::sync::Arc;

use application::domains::env::UpdateEnvData;
use chrono::Utc;
use http_problem::{http::bad_request, Result};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domains::env::{CreateEnvData, Env},
    repositories::env::EnvRepository,
};

pub type DynEnvHandler = Arc<dyn EnvHandler + Send + Sync>;

#[async_trait::async_trait]
pub trait EnvHandler {
    async fn list(&self) -> Result<Vec<Env>>;
    async fn create(&self, data: CreateEnvData) -> Result<Env>;
    async fn update(&self, id: Uuid, data: UpdateEnvData) -> Result<Env>;
}

pub struct DefaultEnvHandler {
    pub env_repository: Arc<dyn EnvRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl EnvHandler for DefaultEnvHandler {
    async fn list(&self) -> Result<Vec<Env>> {
        Ok(self.env_repository.get_all().await?)
    }

    async fn create(&self, data: CreateEnvData) -> Result<Env> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let env = Env {
            id: Uuid::new_v4(),
            name: data.name,
            created_at: Utc::now().naive_local(),
        };
        self.env_repository.insert(&env).await?;

        log::info!("created env `{}`", &env.name);

        Ok(env)
    }

    async fn update(&self, id: Uuid, data: UpdateEnvData) -> Result<Env> {
        data.validate()
            .map_err(|err| bad_request(err.to_string()))?;

        let mut env = self.env_repository.get_by_id(&id).await?;

        if let Some(name) = data.name {
            env.name = name;
        }

        self.env_repository.insert(&env).await?;

        log::info!("edited env `{}`", &env.name);

        Ok(env)
    }
}
