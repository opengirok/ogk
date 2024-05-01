use base64::{engine::general_purpose, Engine as _};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub default: AuthUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
}

impl AuthConfig {
    pub fn new(username: &str, password: &str) -> Self {
        AuthConfig {
            default: AuthUser::new(username, password),
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

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(self)?;
        let realopen_path = AuthConfig::root_path();
        let file_path = AuthConfig::credential_file_path();
        create_dir_all(Path::new(&realopen_path))?;
        let mut local_file = File::create(Path::new(&file_path))?;
        local_file.write_all(toml.as_bytes())?;
        Ok(())
    }
}

impl AuthUser {
    pub fn new(username: &str, password: &str) -> Self {
        AuthUser {
            username: username.to_owned(),
            password: general_purpose::STANDARD.encode(password.as_bytes()),
        }
    }
}
