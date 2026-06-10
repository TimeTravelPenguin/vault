use std::path::PathBuf;

use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;

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
    pub async fn new(database: PathBuf) -> Result<Self, StoreError> {
        let db =
            Database::connect(format!("sqlite://{}?mode=rwc", database.to_string_lossy())).await?;

        db.get_schema_registry("vault_db::entity::*")
            .sync(&db)
            .await?;

        Ok(Self { db })
    }
}
