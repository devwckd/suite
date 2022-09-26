use http_problem::{
    http::{conflict, not_found},
    Result,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domains::blueprint::Blueprint;

// pub type DynBlueprintRepository = Arc<dyn BlueprintRepository + Send + Sync>;

#[async_trait::async_trait]
pub trait BlueprintRepository {
    async fn get_all(&self) -> Result<Vec<Blueprint>>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Blueprint>;
    async fn get_by_name(&self, name: &String) -> Result<Blueprint>;
    async fn insert(&self, blueprint: &Blueprint) -> Result<()>;
    async fn upsert(&self, blueprint: &Blueprint) -> Result<()>;
}

pub struct MemoryBlueprintRepository {
    blueprints: RwLock<Vec<Blueprint>>,
}

#[async_trait::async_trait]
impl BlueprintRepository for MemoryBlueprintRepository {
    async fn get_all(&self) -> Result<Vec<Blueprint>> {
        let blueprints = self.blueprints.read().await;

        Ok(blueprints.clone())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Blueprint> {
        let blueprints = self.blueprints.read().await;

        blueprints
            .iter()
            .filter(|blueprint| &blueprint.id == id)
            .next()
            .cloned()
            .ok_or(not_found("blueprint", id))
    }

    async fn get_by_name(&self, name: &String) -> Result<Blueprint> {
        let blueprints = self.blueprints.read().await;

        blueprints
            .iter()
            .filter(|blueprint| blueprint.name.to_lowercase() == name.to_lowercase())
            .next()
            .cloned()
            .ok_or(not_found("blueprint", name))
    }

    async fn insert(&self, blueprint: &Blueprint) -> Result<()> {
        let mut blueprints = self.blueprints.write().await;

        if blueprints
            .iter()
            .filter(|inner_blueprint| inner_blueprint.id == blueprint.id)
            .next()
            .is_some()
        {
            return Err(conflict(format!(
                "blueprint already exists with id {}",
                blueprint.id
            )));
        }

        if blueprints
            .iter()
            .filter(|inner_blueprint| inner_blueprint.name == blueprint.name)
            .next()
            .is_some()
        {
            return Err(conflict(format!(
                "blueprint already exists with name {}",
                blueprint.name
            )));
        }

        blueprints.push(blueprint.clone());
        Ok(())
    }

    async fn upsert(&self, blueprint: &Blueprint) -> Result<()> {
        let mut blueprints = self.blueprints.write().await;

        blueprints.drain_filter(|inner_blueprint| {
            inner_blueprint.id == blueprint.id || inner_blueprint.name == blueprint.name
        });
        blueprints.push(blueprint.clone());

        Ok(())
    }
}

impl Default for MemoryBlueprintRepository {
    fn default() -> Self {
        Self {
            blueprints: RwLock::new(Vec::new()),
        }
    }
}
