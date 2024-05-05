#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate tracing;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use kraken_rest_client::Client;

mod autobuy;
mod autostake;
mod common;

#[derive(Debug, clap::Args, Clone)]
struct CommonArgs {
    #[arg(long, env = "KRAKEN_API_KEY")]
    api_key: String,
    #[arg(long, env = "KRAKEN_API_SECRET")]
    api_secret: String,

    #[arg(short = 'n', long, env = "KRAKEN_CHORES_DRY_RUN")]
    dry_run: bool,
}

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Args {
    #[command(flatten)]
    common_args: CommonArgs,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
enum Command {
    Autostake(autostake::Args),
    Autobuy(autobuy::Args),
}

#[tokio::main]
async fn main() {
        let dotenv_result = dotenv();
    env_logger::init();

    match dotenv_result {
        Ok(path) => info!("Loaded env file: {}", path.display()),
        Err(e) => error!("Failed to load env file: {}", e),
    }

    let args = Args::parse();

    if args.common_args.dry_run {
        info!("Running in dry-run mode")
    }

    let client = Client::builder()
        .api_key(args.common_args.api_key.as_str())
        .api_secret(args.common_args.api_secret.as_str())
        .build();

    match args.command {
        Command::Autostake(subargs) => autostake::run(&client, subargs).await.unwrap(),
        Command::Autobuy(subargs) => autobuy::run(&client, subargs, args.common_args)
            .await
            .unwrap(),
    }
}
