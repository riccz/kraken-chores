#[macro_use]
extern crate anyhow;

use std::env;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use kraken_rest_client::Client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod autostake;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(env = "KRAKEN_API_KEY")]
    api_key: String,
    #[arg(env = "KRAKEN_API_SECRET")]
    api_secret: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
enum Command {
    Autostake(autostake::Args),
}

#[tokio::main]
async fn main() {
    let dotenv_result = dotenv();
    let subscribers = tracing_subscriber::FmtSubscriber::builder().finish();

    let args = Args::parse();

    let client = Client::builder()
        .api_key(args.api_key)
        .api_secret(args.api_secret)
        .build();

    match args.command {
        Command::Autostake(args) => autostake::run(&client, args).await.unwrap(),
    }
}
