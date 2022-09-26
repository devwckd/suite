use http_problem::{
    http::{conflict, not_found},
    Result,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domains::unit::Unit;

#[async_trait::async_trait]
pub trait UnitRepository {
    async fn get_all(&self) -> Result<Vec<Unit>>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Unit>;
    async fn insert(&self, unit: &Unit) -> Result<()>;
    async fn upsert(&self, unit: &Unit) -> Result<()>;
}

pub struct MemoryUnitRepository {
    units: RwLock<Vec<Unit>>,
}

#[async_trait::async_trait]
impl UnitRepository for MemoryUnitRepository {
    async fn get_all(&self) -> Result<Vec<Unit>> {
        let units = self.units.read().await;

        Ok(units.clone())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Unit> {
        let units = self.units.read().await;

        units
            .iter()
            .filter(|inner_unit| &inner_unit.id == id)
            .next()
            .cloned()
            .ok_or(not_found("unit", id))
    }

    async fn insert(&self, unit: &Unit) -> Result<()> {
        let mut units = self.units.write().await;

        if units
            .iter()
            .filter(|inner_unit| inner_unit.id == unit.id)
            .next()
            .is_some()
        {
            return Err(conflict(format!("unit already exists with id {}", unit.id)));
        }

        units.push(unit.clone());

        Ok(())
    }

    async fn upsert(&self, unit: &Unit) -> Result<()> {
        let mut units = self.units.write().await;

        units.drain_filter(|inner_unit| inner_unit.id == unit.id);
        units.push(unit.clone());

        Ok(())
    }
}

impl Default for MemoryUnitRepository {
    fn default() -> Self {
        Self {
            units: RwLock::new(Vec::new()),
        }
    }
}
