use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // files
    pub local_file_repository: Option<String>,
    pub remote_file_repository: Option<String>,

    // sync
    pub supabase_host: Option<String>,
    pub supabase_api_key: Option<String>,

    // integration
    pub slack_webhook_url: Option<String>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _local_file_repository = match &self.local_file_repository {
            Some(rr) => format!("{}", rr),
            None => Config::default_local_repository(),
        };

        let _remote_file_repository = match &self.remote_file_repository {
            Some(rr) => format!("{}", rr),
            None => format!("⚠️  NOT CONFIGURED ⚠️"),
        };

        let _supabase_host = match &self.supabase_host {
            Some(rr) => format!("{}", rr),
            None => format!("⚠️  NOT CONFIGURED ⚠️"),
        };

        let _supabase_api_key = match &self.supabase_api_key {
            Some(rr) => format!("{}", rr),
            None => format!("⚠️  NOT CONFIGURED ⚠️"),
        };

        let _slack_webhook_url = match &self.slack_webhook_url {
            Some(rr) => format!("{}", rr),
            None => format!("⚠️  NOT CONFIGURED ⚠️"),
        };

        write!(
      f,
      "🗄  FILES:\nlocal file repository: {}\nremote file repository(github): {}\n\n💾 DATABASE(supabase)\nhost: {}\napi_key: {}\n\n🔌 INTEGRATION\nSLACK WEBHOOK URL: {}",
      _local_file_repository, _remote_file_repository, _supabase_host, _supabase_api_key, _slack_webhook_url
    )
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            local_file_repository: Some(Config::default_local_repository()),
            remote_file_repository: None,

            supabase_host: None,
            supabase_api_key: None,

            slack_webhook_url: None,
        }
    }

    pub fn _load() -> Result<Config, Box<dyn Error>> {
        let file_path = Config::file_path();
        let config_file = read_to_string(file_path)?;
        let config = toml::from_str(&config_file)?;
        Ok(config)
    }

    pub fn load_or_new() -> Result<Config, Box<dyn Error>> {
        let file_path = Config::file_path();
        match read_to_string(file_path) {
            Ok(config_file) => {
                let config = toml::from_str(&config_file)?;
                return Ok(config);
            }
            Err(_) => {
                return Ok(Config::new());
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(self)?;
        let realopen_path = Config::root_path();
        let file_path = Config::file_path();
        create_dir_all(Path::new(&realopen_path))?;
        let mut local_file = File::create(Path::new(&file_path))?;
        local_file.write_all(toml.as_bytes())?;
        Ok(())
    }

    pub fn file_path() -> String {
        format!("{}/{}", Config::root_path(), "config")
    }

    pub fn default_local_repository() -> String {
        format!("{}/{}", Config::root_path(), ".data")
    }

    pub fn root_path() -> String {
        format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".ogk")
    }
}
