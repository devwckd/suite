use application::domains::blueprint::Blueprint;

use crate::BASE_URL;

#[derive(clap::Parser)]
#[clap(name = "Blueprint")]
#[clap(about = "Does awesome things", long_about = None)]
pub struct BlueprintCommand {
    #[clap(subcommand)]
    command: BlueprintSubCommand,
}

#[derive(clap::Subcommand)]
pub enum BlueprintSubCommand {
    List,
}

pub async fn exec(blueprint_command: BlueprintCommand) {
    match blueprint_command.command {
        BlueprintSubCommand::List => exec_list().await,
    }
}

async fn exec_list() {
    let blueprints = reqwest::get(format!("{}/blueprints/", BASE_URL))
        .await
        .unwrap()
        .json::<Vec<Blueprint>>()
        .await
        .unwrap();

    dbg!(&blueprints);
}
