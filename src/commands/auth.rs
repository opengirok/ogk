use std::cell::RefCell;

use crate::client;
use crate::utils::auth::{AuthConfig, AuthUser};
use clap::Subcommand;

#[derive(Subcommand)]
#[clap(about = "(required) Authenticate for open.go.kr", author, long_about = None, version)]
pub enum Commands {
    #[clap(about = "Login on open.go.kr with a valid account")]
    Login {
        #[clap(long = "org")]
        org: String,

        #[clap(short = 'u', long = "username")]
        username: Option<String>,

        #[clap(short = 'p', long = "password")]
        password: Option<String>,
    },
    #[clap(about = "list of accounts stored before")]
    List {},
}

async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let auth_config = AuthConfig::load().unwrap();
    println!("{:?}", auth_config);

    Ok(())
}

async fn login_with_username(
    org: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::Client::new().await?;
    let _ = client.auth(username, password).await?;

    let config = AuthConfig::load_or_new().unwrap();
    let _ = config.add_account(org, username, password);
    Ok(())
}

async fn login_with_auth_user(
    auth_config: &RefCell<AuthUser>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::Client::new().await?;
    let _ = client
        .auth(
            &auth_config.borrow().username,
            &auth_config.borrow().get_decoded_password(),
        )
        .await?;

    Ok(())
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        Commands::Login {
            org,
            username,
            password,
        } => {
            let auth_config = AuthConfig::load_or_new().unwrap();
            match auth_config.find_org(org) {
                Some(auth_user) => {
                    if username.is_none() || password.is_none() {
                        let _result = login_with_auth_user(auth_user.to_owned()).await;
                        return Ok(());
                    } else {
                        let _username = username.as_ref().expect("username is required");
                        let _password = password.as_ref().expect("password is required");
                        let _result = login_with_username(&org, &_username, &_password).await;
                        return Ok(());
                    }
                }
                None => {
                    if username.is_none() || password.is_none() {
                        println!(
                            "이전에 저장된 로그인 정보가 없는 조직명입니다. 로그인 정보를 입력해주세요."
                        );
                        return Ok(());
                    }
                    let _username = username.as_ref().expect("username is required");
                    let _password = password.as_ref().expect("password is required");
                    let _result = login_with_username(&org, &_username, &_password).await;
                }
            }
        }
        Commands::List {} => {
            let _result = list().await;
        }
    }

    Ok(())
}
