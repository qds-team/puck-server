use crate::db::models::{Settings, DatabaseSettings, ServerSettings};
use std::fs;
use toml;
use serde_derive::Deserialize;
use serde::{ser, de};
use std::fmt::{self, Display};

#[derive(Deserialize, Debug)]
pub enum EnvError {
    Message(String),
    UnableToLoadEnvFile,
}

impl de::Error for EnvError {
    fn custom<T: Display>(msg: T) -> Self {
        EnvError::Message(msg.to_string())
    }
}

impl Display for EnvError{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvError::Message(msg) => formatter.write_str(msg),
            EnvError::UnableToLoadEnvFile => formatter.write_str("unexpected end of input"),
        }
    }
}

impl std::error::Error for EnvError {}

fn load_config() -> Result<Settings, EnvError> {

    let config = "../../env.toml";
    let config_contents = match fs::read_to_string(config) {
        Ok(c) => c,
        Err(e) => e.to_string(), 
    };

    let config_data: Result<Settings, EnvError> = match toml::from_str(&config_contents) {
        Ok(d) => d,

        Err(e) => Err(EnvError::UnableToLoadEnvFile),
    };

    return config_data;
}

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

pub fn set_password(password_hash: String) {
    /*TODO: Set Password here */

}
