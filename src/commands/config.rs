use crate::utils::config::Config;
use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand)]
#[clap(about = "Configurations", author, long_about = None, version)]
pub enum Commands {
    #[clap(about = "Display all config list")]
    List,
    #[clap(about = "Configuration to manage files")]
    Files {
        #[clap(long = "local-repository", required = false)]
        local_repository: Option<String>,
        #[clap(long = "remote-repository", required = false)]
        remote_repository: Option<String>,
    },
    #[clap(about = "Configuration to sync data")]
    Sync {
        #[clap(long = "supabase-host", required = false)]
        supabase_host: Option<String>,
        #[clap(long = "supabase-api-key", required = false)]
        supabase_api_key: Option<String>,
    },
    #[clap(about = "Configuration for integration")]
    Integration {
        #[clap(long = "slack-webhook-url", required = false)]
        slack_webhook_url: Option<String>,
    },
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
    match args {
        Commands::List => {
            let config = Config::load_or_new()?;
            println!("{}", config);
        }
        Commands::Sync {
            supabase_api_key,
            supabase_host,
        } => {
            let mut config = Config::load_or_new()?;

            match supabase_api_key {
                Some(rr) => {
                    config.supabase_api_key = Some(rr.to_string());
                }
                None => {}
            }

            match supabase_host {
                Some(lr) => {
                    config.supabase_host = Some(lr.to_string());
                }
                None => {}
            }

            config.save()?;
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
        Commands::Integration { slack_webhook_url } => {
            let mut config = Config::load_or_new()?;

            match slack_webhook_url {
                Some(rr) => {
                    config.slack_webhook_url = Some(rr.to_string());
                }
                None => {}
            }

            config.save()?;
        }
    }

    Ok(())
}
