use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(vault_migrations::Migrator).await;
}
