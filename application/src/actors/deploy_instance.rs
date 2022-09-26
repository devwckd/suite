use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    domains::{
        blueprint::Blueprint, env::Env, instance::Instance, unit::CreateUnitData, version::Version,
    },
    handlers::unit::UnitHandler,
};

struct DeployUnitActor {
    receiver: Receiver<(Blueprint, Env, Instance, Version)>,
    unit_handler: Arc<dyn UnitHandler + Send + Sync>,
}

impl DeployUnitActor {
    async fn handle(
        &self,
        (blueprint, env, instance, version): (Blueprint, Env, Instance, Version),
    ) {
        let unit_handler = self.unit_handler.clone();
        tokio::spawn(async move {
            let mut env_variables = Vec::new();
            env_variables.push(format!("ENV={}", &env.name));
            env_variables.push(format!("INSTANCE={}", &instance.name));
            env_variables.push(format!("INSTANCE_ID={}", &instance.id));
            env_variables.push(format!("VERSION={}", &version.version));
            env_variables.push(format!("VERSION_ID={}", &version.id));
            env_variables.push("EULA=TRUE".to_string());

            let data = CreateUnitData {
                image: format!("{}:{}", &blueprint.image, &version.version),
                env_variables: env_variables,
                exposed_port: Some(25565),
            };

            let unit = unit_handler.create(data).await;
            log::info!("{:?}", unit);
        });
    }
}

#[derive(Clone)]
pub struct DeployUnitHandle {
    sender: Sender<(Blueprint, Env, Instance, Version)>,
}

impl DeployUnitHandle {
    pub fn new(unit_handler: Arc<dyn UnitHandler + Send + Sync>) -> Self {
        let (sender, receiver) = channel(8);
        let actor = DeployUnitActor {
            receiver,
            unit_handler,
        };

        tokio::spawn(run_deploy_unit_actor(actor));
        Self { sender }
    }

    pub async fn deploy(
        &self,
        blueprint: &Blueprint,
        env: &Env,
        instance: &Instance,
        version: &Version,
    ) {
        self.sender
            .send((
                blueprint.clone(),
                env.clone(),
                instance.clone(),
                version.clone(),
            ))
            .await
            .unwrap();
    }
}

async fn run_deploy_unit_actor(mut actor: DeployUnitActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle(msg).await;
    }
}
