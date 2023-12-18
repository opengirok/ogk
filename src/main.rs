#![allow(unused_must_use)]

mod client;
mod commands;
mod database;
mod files;
mod utils;

use clap::Parser;
use dotenv::dotenv;
use std::error::Error;

#[derive(Parser)]
#[clap(name = "ogk")]
#[clap(about = "cli for open.go.kr", long_about = None)]
#[clap(author = "pretty00butt@protonmail.com", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: commands::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Cli::parse();
    let _result = commands::run(&args.command).await;
    Ok(())
}
