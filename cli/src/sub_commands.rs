pub mod blueprint;
pub mod env;

use self::{blueprint::BlueprintCommand, env::EnvCommand};

#[derive(clap::Subcommand)]
pub enum SubCommand {
    Blueprint(BlueprintCommand),
    Env(EnvCommand),
}
