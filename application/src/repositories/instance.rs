use http_problem::{
    http::{conflict, not_found},
    Result,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domains::instance::Instance;

#[async_trait::async_trait]
pub trait InstanceRepository {
    async fn get_all(&self) -> Result<Vec<Instance>>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Instance>;
    async fn get_by_name(&self, name: &String) -> Result<Instance>;
    async fn insert(&self, instance: &Instance) -> Result<()>;
    async fn upsert(&self, instance: &Instance) -> Result<()>;
}

pub struct MemoryInstanceRepository {
    instances: RwLock<Vec<Instance>>,
}

#[async_trait::async_trait]
impl InstanceRepository for MemoryInstanceRepository {
    async fn get_all(&self) -> Result<Vec<Instance>> {
        let instances = self.instances.read().await;

        Ok(instances.clone())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Instance> {
        let instances = self.instances.read().await;

        instances
            .iter()
            .filter(|instance| &instance.id == id)
            .next()
            .cloned()
            .ok_or(not_found("instance", id))
    }

    async fn get_by_name(&self, name: &String) -> Result<Instance> {
        let instances = self.instances.read().await;

        instances
            .iter()
            .filter(|instance| &instance.name == name)
            .next()
            .cloned()
            .ok_or(not_found("instance", name))
    }

    async fn insert(&self, instance: &Instance) -> Result<()> {
        let mut instances = self.instances.write().await;

        if instances
            .iter()
            .filter(|inner_instance| inner_instance.id == instance.id)
            .next()
            .is_some()
        {
            return Err(conflict(format!(
                "instance already exists with id {}",
                instance.id
            )));
        }

        if instances
            .iter()
            .filter(|inner_instance| inner_instance.name == instance.name)
            .next()
            .is_some()
        {
            return Err(conflict(format!(
                "instance already exists with name {}",
                instance.name
            )));
        }

        instances.push(instance.clone());

        Ok(())
    }

    async fn upsert(&self, instance: &Instance) -> Result<()> {
        let mut instances = self.instances.write().await;

        instances.drain_filter(|inner_instance| {
            inner_instance.id == instance.id || inner_instance.name == inner_instance.name
        });
        instances.push(instance.clone());

        Ok(())
    }
}

impl Default for MemoryInstanceRepository {
    fn default() -> Self {
        Self {
            instances: RwLock::new(Vec::new()),
        }
    }
}
