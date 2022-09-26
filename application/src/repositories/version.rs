use http_problem::{http::not_found, Result};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domains::version::Version;

#[async_trait::async_trait]
pub trait VersionRepository {
    async fn get_all(&self) -> Result<Vec<Version>>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Version>;
    async fn insert(&self, version: &Version) -> Result<()>;
}

pub struct MemoryVersionRepository {
    versions: RwLock<Vec<Version>>,
}

#[async_trait::async_trait]
impl VersionRepository for MemoryVersionRepository {
    async fn get_all(&self) -> Result<Vec<Version>> {
        let versions = self.versions.read().await;

        Ok(versions.clone())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Version> {
        let versions = self.versions.read().await;

        versions
            .iter()
            .filter(|version| &version.id == id)
            .next()
            .cloned()
            .ok_or(not_found("version", id))
    }

    async fn insert(&self, version: &Version) -> Result<()> {
        let mut versions = self.versions.write().await;

        versions.push(version.clone());

        Ok(())
    }
}

impl Default for MemoryVersionRepository {
    fn default() -> Self {
        Self {
            versions: RwLock::new(Vec::new()),
        }
    }
}
