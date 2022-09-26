use application::domains::env::{CreateEnvData, Env};

use crate::BASE_URL;

#[derive(clap::Parser)]
pub struct EnvCommand {
    #[clap(subcommand)]
    command: EnvSubCommand,
}

#[derive(clap::Subcommand)]
pub enum EnvSubCommand {
    List,
    Create { name: String },
}

pub async fn exec(env_command: EnvCommand) {
    match env_command.command {
        EnvSubCommand::List => exec_list().await,
        EnvSubCommand::Create { name } => exec_create(name).await,
    }
}

async fn exec_list() {
    let envs = reqwest::get(format!("{}/envs/", BASE_URL))
        .await
        .unwrap()
        .json::<Vec<Env>>()
        .await
        .unwrap();

    dbg!(&envs);
}

async fn exec_create(name: String) {
    let client = reqwest::Client::new();

    let env = client
        .post(format!("{}/envs/", BASE_URL))
        .json(&CreateEnvData { name })
        .send()
        .await
        .unwrap()
        .json::<Env>()
        .await
        .unwrap();

    dbg!(&env);
}
