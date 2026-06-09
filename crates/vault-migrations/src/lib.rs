pub use sea_orm_migration::prelude::*;

use crate::migrations::m20260609_initialise;

mod migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260609_initialise::Migration)]
    }
}
