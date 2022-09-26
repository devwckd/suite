mod sub_commands;

use clap::Parser;
use sub_commands::SubCommand;

pub const BASE_URL: &'static str = "http://localhost:3000";

#[derive(clap::Parser)]
#[clap(name = "MyApp")]
#[clap(author = "João Victor G. Cruz <dev.wckd@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Does awesome things", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Blueprint(bp) => sub_commands::blueprint::exec(bp).await,
        SubCommand::Env(env) => sub_commands::env::exec(env).await,
    }
}
