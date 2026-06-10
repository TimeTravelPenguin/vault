use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;

use crate::config::DatabaseLocation;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

#[derive(Debug)]
pub struct VaultStore {
    db: DatabaseConnection,
}

impl VaultStore {
    pub async fn new(db_location: DatabaseLocation) -> Result<Self, StoreError> {
        let db = match db_location {
            DatabaseLocation::FilePath(path) => {
                Database::connect(format!("sqlite://{}", path.to_string_lossy())).await?
            }
            DatabaseLocation::ConnectionString(url) => {
                Database::connect(url.as_url().as_str()).await?
            }
        };

        Ok(Self { db })
    }
}
