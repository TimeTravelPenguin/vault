use sea_orm_migration::prelude::*;
use vault_db::entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.get_schema_builder()
            .register(entity::directories::Entity)
            .register(entity::documents::Entity)
            .register(entity::tags::Entity)
            .register(entity::document_tags::Entity)
            .register(entity::document_relationships::Entity)
            .apply(db)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
