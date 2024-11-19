use base64::{engine::general_purpose, Engine as _};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;
use std::str;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub accounts: HashMap<String, RefCell<AuthUser>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub org: String,
    pub username: String,
    pub password: String,
    pub local_repository: Option<String>,
    pub remote_repository: Option<String>,
    pub slack_webhook_url: Option<String>,
}

impl AuthConfig {
    pub fn new() -> Self {
        AuthConfig {
            accounts: HashMap::new(),
        }
    }

    pub fn root_path() -> String {
        format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".ogk")
    }

    pub fn credential_file_path() -> String {
        format!("{}/{}", AuthConfig::root_path(), "credentials")
    }

    pub fn load() -> Result<AuthConfig, Box<dyn Error>> {
        let file_path = AuthConfig::credential_file_path();
        let credential_file = read_to_string(file_path)?;
        let credential = toml::from_str(&credential_file)?;
        Ok(credential)
    }

    pub fn load_or_new() -> Result<AuthConfig, Box<dyn Error>> {
        let file_path = AuthConfig::credential_file_path();

        match read_to_string(file_path) {
            Ok(credential_file) => {
                let credential = toml::from_str(&credential_file)?;
                return Ok(credential);
            }
            Err(_) => {
                return Ok(AuthConfig::new());
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(self)?;
        let realopen_path = AuthConfig::root_path();
        let file_path = AuthConfig::credential_file_path();
        create_dir_all(Path::new(&realopen_path))?;
        let mut local_file = File::create(Path::new(&file_path))?;
        local_file.write_all(toml.as_bytes())?;
        Ok(())
    }

    pub fn add_account(
        &self,
        org: &str,
        username: &str,
        password: &str,
    ) -> Result<AuthConfig, Box<dyn Error>> {
        let mut auth_config = AuthConfig::load_or_new().unwrap();

        if auth_config.accounts.len() == 0 {
            auth_config.accounts.insert(
                String::from("default"),
                RefCell::new(AuthUser::new(org, username, password)),
            );

            auth_config.save();
        }

        auth_config.accounts.insert(
            org.to_owned(),
            RefCell::new(AuthUser::new(org, username, password)),
        );

        auth_config.save();

        Ok(auth_config)
    }

    pub fn find_org(&self, org: &str) -> Option<&RefCell<AuthUser>> {
        self.accounts.get(org)
    }

    pub fn set_remote_repository_path(
        &self,
        org: &str,
        remote_repository: &str,
    ) -> Result<AuthConfig, Box<dyn Error>> {
        let auth_config = AuthConfig::load_or_new().unwrap();
        if let Some(value_refcell) = auth_config.accounts.get(org) {
            let mut option = value_refcell.borrow_mut();
            option.remote_repository = Some(remote_repository.to_string());
        }

        if let Some(value_refcell) = auth_config.accounts.get("default") {
            let mut option = value_refcell.borrow_mut();
            if option.org == org {
                option.remote_repository = Some(remote_repository.to_string());
            }
        }

        auth_config.save();

        Ok(auth_config)
    }

    pub fn set_local_repository_path(
        &self,
        org: &str,
        local_repository: &str,
    ) -> Result<AuthConfig, Box<dyn Error>> {
        let auth_config = AuthConfig::load_or_new().unwrap();
        if let Some(value_refcell) = auth_config.accounts.get(org) {
            let mut option = value_refcell.borrow_mut();
            option.local_repository = Some(local_repository.to_string());
        }

        if let Some(value_refcell) = auth_config.accounts.get("default") {
            let mut option = value_refcell.borrow_mut();
            if option.org == org {
                option.local_repository = Some(local_repository.to_string());
            }
        }

        auth_config.save();

        Ok(auth_config)
    }

    pub fn set_slack_webhook_url(
        &self,
        org: &str,
        url: &str,
    ) -> Result<AuthConfig, Box<dyn Error>> {
        let auth_config = AuthConfig::load_or_new().unwrap();
        if let Some(value_refcell) = auth_config.accounts.get(org) {
            let mut option = value_refcell.borrow_mut();
            option.slack_webhook_url = Some(url.to_string());
        }

        if let Some(value_refcell) = auth_config.accounts.get("default") {
            let mut option = value_refcell.borrow_mut();
            if option.org == org {
                option.slack_webhook_url = Some(url.to_string());
            }
        }

        auth_config.save();

        Ok(auth_config)
    }
}

impl AuthUser {
    fn encode_password(password: &str) -> String {
        general_purpose::STANDARD.encode(password.as_bytes())
    }

    pub fn new(org: &str, username: &str, password: &str) -> Self {
        AuthUser {
            org: org.to_owned(),
            username: username.to_owned(),
            password: AuthUser::encode_password(password),
            remote_repository: None,
            local_repository: None,
            slack_webhook_url: None,
        }
    }

    pub fn get_decoded_password(&self) -> String {
        let decoded_password = general_purpose::STANDARD
            .decode(&self.password.as_bytes())
            .unwrap();

        str::from_utf8(&decoded_password).unwrap().to_owned()
    }
}
