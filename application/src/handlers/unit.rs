use std::{collections::HashMap, sync::Arc};

use bollard::{container::StartContainerOptions, Docker};
use http_problem::{http::internal_error, Result};
use uuid::Uuid;

use crate::{
    domains::unit::{CreateUnitData, Unit},
    repositories::unit::UnitRepository,
};

#[async_trait::async_trait]
pub trait UnitHandler {
    async fn create(&self, data: CreateUnitData) -> Result<Unit>;
}

pub struct DockerUnitHandler {
    pub docker_api: Docker,
    pub unit_repository: Arc<dyn UnitRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl UnitHandler for DockerUnitHandler {
    async fn create(&self, data: CreateUnitData) -> Result<Unit> {
        let config = bollard::container::Config {
            image: Some(data.image.clone()),
            env: Some(data.env_variables.clone()),
            exposed_ports: data.exposed_port.map(|port| {
                let mut ports = HashMap::new();
                ports.insert(format!("{}/tcp", port), HashMap::new());

                ports
            }),
            ..Default::default()
        };

        let container = self
            .docker_api
            .create_container::<String, _>(None, config)
            .await
            .map_err(|err| internal_error(err))?;

        self.docker_api
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .unwrap();

        let unit = Unit {
            id: Uuid::new_v4(),
            container: container.id.clone(),
            image: data.image.clone(),
            env_variables: data.env_variables.clone(),
            exposed_port: data.exposed_port.clone(),
        };

        self.unit_repository.insert(&unit).await?;

        Ok(unit)
    }
}
