use clap::Parser;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait, entity};

use thiserror::Error;
use vault_app::{cli, tui::TuiError};
use vault_migrations::MigratorTrait;

#[derive(Debug, Error)]
enum AppError {
    #[error("TUI error: {0}")]
    Tui(#[from] TuiError),
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("ColorEyre error: {0}")]
    ColorEyre(#[from] color_eyre::eyre::Report),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("The specified database path is a directory: {0}")]
    DbPathIsDirectory(String),
}

type Result<T> = std::result::Result<T, AppError>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Tui(args) => {
            run_tui(args).await?;
        }
        cli::Commands::Db(command) => match command {
            cli::DbSubcommand::Create(args) => {
                create_db(args).await?;
            }
            cli::DbSubcommand::Migrate(args) => {
                migrate_db(args).await?;
            }
        },
        cli::Commands::Test => {
            test().await?;
        }
    };

    Ok(())
}

async fn test() -> Result<()> {
    color_eyre::install()?;

    std::fs::remove_file("db.sqlite").ok();
    let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
    db.get_schema_registry("vault_db::entity::*")
        .sync(&db)
        .await?;

    let tag = vault_db::tags::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
        name: ActiveValue::Set("Test Tag".to_string()),
        ..Default::default()
    };

    let res = vault_db::tags::Entity::insert(tag).exec(&db).await?;
    println!("{:?}", res);

    Ok(())
}

async fn run_tui(args: cli::TuiArgs) -> Result<()> {
    color_eyre::install()?;

    let db = Database::connect(format!(
        "sqlite://{}?mode=rwc",
        args.db_args.path.to_string_lossy()
    ))
    .await?;

    vault_app::tui::run(db).map_err(AppError::Tui)
}

async fn create_db(args: cli::DbCreateArgs) -> Result<()> {
    let db_path = args.db_args.path;
    let replace = if args.replace {
        "?mode=rwc"
    } else {
        Default::default()
    };
    let conn_str = format!("sqlite://{}{}", db_path.to_string_lossy(), replace);

    if db_path.is_dir() {
        return Err(AppError::DbPathIsDirectory(
            db_path.to_string_lossy().to_string(),
        ));
    }

    if args.replace && db_path.exists() {
        println!("Removing existing database file: {:?}", db_path);
        std::fs::remove_file(&db_path)?;
    }

    let db: DatabaseConnection = Database::connect(&conn_str).await?;

    db.get_schema_registry("vault_db::entity::*")
        .sync(&db)
        .await?;

    println!("Completed database setup");
    Ok(())
}

async fn migrate_db(args: cli::DbMigrateArgs) -> Result<()> {
    let db_path = args.db_args.path;
    let conn_str = format!("sqlite://{}", db_path.to_string_lossy());

    if db_path.is_dir() {
        return Err(AppError::DbPathIsDirectory(
            db_path.to_string_lossy().to_string(),
        ));
    }

    let db: DatabaseConnection = Database::connect(&conn_str).await?;

    vault_migrations::Migrator::up(&db, None).await?;

    println!("Completed database migration");
    Ok(())
}
