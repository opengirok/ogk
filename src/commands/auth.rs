use crate::client;
use crate::utils::auth::AuthConfig;
use clap::{AppSettings, Subcommand};

#[derive(Subcommand)]
#[clap(about = "(required) Authenticate for open.go.kr", author, long_about = None, version)]
pub enum Commands {
    #[clap(
        about = "Login on open.go.kr with a valid account",
        setting(AppSettings::ArgRequiredElseHelp)
    )]
    Login {
        #[clap(short = 'u', long = "username")]
        username: String,

        #[clap(short = 'p', long = "password")]
        password: String,
    },
}

async fn login(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::Client::new();
    let _ = client.auth(username, password).await?;

    let config = AuthConfig::new(username, password);
    let _ = config.save()?;

    Ok(())
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        Commands::Login { username, password } => {
            let _result = login(&username, &password).await;
        }
    }

    Ok(())
}
