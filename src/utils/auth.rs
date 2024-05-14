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
            // password: general_purpose::STANDARD.encode(password.as_bytes()),
        }
    }

    pub fn get_decoded_password(&self) -> String {
        let decoded_password = general_purpose::STANDARD
            .decode(&self.password.as_bytes())
            .unwrap();

        str::from_utf8(&decoded_password).unwrap().to_owned()
    }
}
