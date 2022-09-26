use crate::domains::{blueprint::Blueprint, env::Env, instance::Instance};
use bollard::{container::CreateContainerOptions, Docker};
use http_problem::Result;

#[async_trait::async_trait]
pub trait Maestro {
    async fn deploy(&self, blueprint: &Blueprint, env: &Env, instance: &Instance)
        -> Result<String>;
}

pub struct DockerMaestro {
    pub docker: Docker,
}

#[async_trait::async_trait]
impl Maestro for DockerMaestro {
    async fn deploy(
        &self,
        blueprint: &Blueprint,
        env: &Env,
        instance: &Instance,
    ) -> Result<String> {
        let options = CreateContainerOptions {
            name: instance.name.clone(),
        };

        let env_variables = vec![format!("ENVIRONMENT={}", &env.name)];

        let config = bollard::container::Config {
            image: Some(blueprint.image.clone()),
            env: Some(env_variables),
            ..Default::default()
        };

        let container = self
            .docker
            .create_container(Some(options), config)
            .await
            .unwrap();

        Ok(container.id)
    }
}
