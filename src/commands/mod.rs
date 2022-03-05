use clap::Subcommand;
use std::error::Error;

pub mod auth;
pub mod config;
pub mod download;
pub mod fetch;
pub mod sync;

#[derive(Subcommand)]
pub enum Commands {
  #[clap(subcommand)]
  Auth(auth::Commands),
  #[clap(subcommand)]
  Config(config::Commands),
  #[clap(subcommand)]
  Download(download::Commands),
  #[clap(subcommand)]
  Fetch(fetch::Commands),
  #[clap(about = "Syncronize data on open.go.kr with Supabase database", author, long_about = None, version)]
  Sync(sync::Commands),
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
  match args {
    Commands::Auth(subcommands) => {
      let _result = auth::run(subcommands).await;
    }
    Commands::Config(subcommands) => {
      let _result = config::run(subcommands).await;
    }
    Commands::Download(subcommands) => {
      let _result = download::run(subcommands).await;
    }
    Commands::Fetch(subcommands) => {
      let _result = fetch::run(subcommands).await;
    }
    Commands::Sync(args) => {
      let _result = sync::run(args).await;
    }
  }

  Ok(())
}
