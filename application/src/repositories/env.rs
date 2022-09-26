use http_problem::{
    http::{conflict, not_found},
    Result,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domains::env::Env;

#[async_trait::async_trait]
pub trait EnvRepository {
    async fn get_all(&self) -> Result<Vec<Env>>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Env>;
    async fn get_by_name(&self, name: &String) -> Result<Env>;
    async fn insert(&self, env: &Env) -> Result<()>;
    async fn upsert(&self, env: &Env) -> Result<()>;
}

pub struct MemoryEnvRepository {
    envs: RwLock<Vec<Env>>,
}

#[async_trait::async_trait]
impl EnvRepository for MemoryEnvRepository {
    async fn get_all(&self) -> Result<Vec<Env>> {
        let envs = self.envs.read().await;

        Ok(envs.clone())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Env> {
        let envs = self.envs.read().await;

        envs.iter()
            .filter(|env| &env.id == id)
            .next()
            .cloned()
            .ok_or(not_found("env", id))
    }

    async fn get_by_name(&self, name: &String) -> Result<Env> {
        let envs = self.envs.read().await;

        envs.iter()
            .filter(|env| env.name.to_lowercase() == name.to_lowercase())
            .next()
            .cloned()
            .ok_or(not_found("env", name))
    }

    async fn insert(&self, env: &Env) -> Result<()> {
        let mut envs = self.envs.write().await;

        if envs
            .iter()
            .filter(|inner_env| inner_env.id == env.id)
            .next()
            .is_some()
        {
            return Err(conflict(format!("env already exists with id {}", env.id)));
        }

        if envs
            .iter()
            .filter(|inner_env| inner_env.name == env.name)
            .next()
            .is_some()
        {
            return Err(conflict(format!(
                "env already exists with name {}",
                env.name
            )));
        }

        envs.push(env.clone());

        Ok(())
    }

    async fn upsert(&self, env: &Env) -> Result<()> {
        let mut envs = self.envs.write().await;

        envs.drain_filter(|inner_env| inner_env.id == env.id || inner_env.name == env.name);
        envs.push(env.clone());

        Ok(())
    }
}

impl Default for MemoryEnvRepository {
    fn default() -> Self {
        Self {
            envs: RwLock::new(Vec::new()),
        }
    }
}
