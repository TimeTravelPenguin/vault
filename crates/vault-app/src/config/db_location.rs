use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Url", into = "Url")]
pub struct DatabaseUrl(Url);

#[derive(Debug, Error)]
pub enum DatabaseUrlError {
    #[error("unsupported database scheme: {0}")]
    UnsupportedScheme(String),

    #[error("database URL must include a host")]
    MissingHost,
}

impl TryFrom<Url> for DatabaseUrl {
    type Error = DatabaseUrlError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        match url.scheme() {
            "postgres" | "postgresql" | "mysql" | "sqlite" => {}
            scheme => return Err(DatabaseUrlError::UnsupportedScheme(scheme.to_owned())),
        }

        // Probably do not require a host for sqlite.
        if url.scheme() != "sqlite" && url.host_str().is_none() {
            return Err(DatabaseUrlError::MissingHost);
        }

        Ok(Self(url))
    }
}

impl From<DatabaseUrl> for Url {
    fn from(value: DatabaseUrl) -> Self {
        value.0
    }
}

impl DatabaseUrl {
    pub fn as_url(&self) -> &Url {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DatabaseLocation {
    /// The database is located at the specified path.
    FilePath(PathBuf),
    /// The database is located at the specified connection string.
    ConnectionString(DatabaseUrl),
}

/// Creates a [`DatabaseLocation`] from a file path.
pub fn from_file_path(path: impl Into<PathBuf>) -> DatabaseLocation {
    DatabaseLocation::FilePath(path.into())
}

/// Tries to create a [`DatabaseLocation`] from a connection string, returning a
/// [`DatabaseUrlError`] if the connection string is invalid or uses an
/// unsupported scheme.
pub fn try_from_connection_string(
    connection_string: impl Into<String>,
) -> Result<DatabaseLocation, DatabaseUrlError> {
    let url = Url::parse(&connection_string.into())
        .map_err(|err| DatabaseUrlError::UnsupportedScheme(format!("invalid URL: {}", err)))?;

    let url = DatabaseUrl::try_from(url)?;
    Ok(DatabaseLocation::ConnectionString(url))
}
