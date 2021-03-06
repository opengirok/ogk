use crate::utils::config::Config;
use clap::{AppSettings, Subcommand};
use std::error::Error;

#[derive(Subcommand)]
#[clap(about = "Configurations", author, long_about = None, version)]
pub enum Commands {
    #[clap(about = "Display all config list")]
    List,
    #[clap(
        about = "Configuration to manage files",
        setting(AppSettings::ArgRequiredElseHelp)
    )]
    Files {
        #[clap(long = "local-repository", required = false)]
        local_repository: Option<String>,
        #[clap(long = "remote-repository", required = false)]
        remote_repository: Option<String>,
    },
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
    match args {
        Commands::List => {
            let config = Config::load_or_new()?;
            println!("{}", config);
        }
        Commands::Files {
            local_repository,
            remote_repository,
        } => {
            let mut config = Config::load_or_new()?;
            match remote_repository {
                Some(rr) => {
                    config.remote_file_repository = Some(rr.to_string());
                }
                None => {}
            }

            match local_repository {
                Some(lr) => {
                    config.local_file_repository = Some(lr.to_string());
                }
                None => {}
            }

            config.save()?;
        }
    }

    Ok(())
}
