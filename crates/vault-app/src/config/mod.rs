use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod config_version;
mod db_location;

pub use config_version::ConfigVersion;

pub const APP_NAME: &str = "vault-app";
pub const CONFIG_FILE_NAME: &str = "config";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[source] std::io::Error),
    #[error("Failed to parse config JSON: {0}")]
    JsonParseError(#[source] serde_json::Error),
    #[error(transparent)]
    ConfyError(#[from] confy::ConfyError),
    #[error("Unsupported config version: {0:?}")]
    UnsupportedVersion(ConfigVersion),
    #[error("Invalid database location: {0}")]
    InvalidDatabaseLocation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: ConfigVersion,
    pub database: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        let config_dir = get_config_path(Some(CONFIG_FILE_NAME))
            .ok()
            .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
            .unwrap_or_else(|| ".".into());

        Self::new(config_dir.join("db.sqlite"))
    }
}

impl AppConfig {
    pub fn new(database: impl Into<PathBuf>) -> Self {
        Self {
            version: ConfigVersion::CURRENT,
            database: database.into(),
        }
    }
}

pub fn get_config_path<'a>(name: impl Into<Option<&'a str>>) -> Result<PathBuf, ConfigError> {
    confy::get_configuration_file_path(APP_NAME, name.into()).map_err(ConfigError::ConfyError)
}

pub fn load_config<'a>(name: impl Into<Option<&'a str>>) -> Result<AppConfig, ConfigError> {
    let config = confy::load(APP_NAME, name.into())?;

    Ok(config)
}

pub fn save_config<'a>(
    config: &AppConfig,
    name: impl Into<Option<&'a str>>,
) -> Result<(), ConfigError> {
    confy::store(APP_NAME, name.into(), config)?;

    Ok(())
}
