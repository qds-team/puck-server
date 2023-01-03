use crate::db::models::{Settings, DatabaseSettings, ServerSettings};
use std::fs;
use toml;
use std::error::Error;

#[derive(Debug)]
pub enum EnvError {
    UnableToLoadEnvFile(Box<dyn Error>),
}

impl From<Box<dyn Error>> for EnvError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::UnableToLoadEnvFile(e)
    }
}

fn load_config() -> Result<Settings, EnvError> /*need to return smthn*/{

    let config = "env.toml";
    let config_contents = match fs::read_to_string(config) {
        Ok(c) => c,
        Err(e) => e.to_string(), 
    };

    let config_data: Result<Settings, EnvError> = match toml::from_str(&config_contents) {
        Ok(d) => d,

        Err(e) => EnvError::UnableToLoadEnvFile,
    };

    return config_data;

pub fn get_password() -> String {
    let settings: Settings = load_config().unwrap();
    return settings.server_settings.password;
}

pub fn get_db_url() -> String {
    let settings: Settings = load_config().unwrap();
    return settings.db_settings.url;
}

pub fn get_universal_path() -> String {
    let settings: Settings = load_config().unwrap();
    return settings.server_settings.universal_path;
}