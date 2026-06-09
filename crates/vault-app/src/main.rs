use anyhow::{Context, Result};
use clap::Parser;
use sea_orm::{Database, DatabaseConnection};

use vault_app::cli;
use vault_migrations::MigratorTrait;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    println!("{:#?}", cli);

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
    };

    Ok(())
}

async fn run_tui(args: cli::TuiArgs) -> Result<()> {
    let db = Database::connect(format!(
        "sqlite://{}?mode=rwc",
        args.db_args.path.to_string_lossy()
    ))
    .await
    .context("Failed to connect to database")?;

    vault_app::app::run(db).context("Error running application")
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
        return Err(anyhow::anyhow!(
            "The specified path is a directory: {:?}",
            db_path
        ));
    }

    if args.replace && db_path.exists() {
        println!("Removing existing database file: {:?}", db_path);
        std::fs::remove_file(&db_path)
            .with_context(|| format!("Failed to remove existing database file: {:?}", db_path))?;
    }

    let db: DatabaseConnection = Database::connect(&conn_str)
        .await
        .context("Failed to connect to database")?;

    db.get_schema_registry("vault_db::entity::*")
        .sync(&db)
        .await
        .context("Failed to synchronize database schema")?;

    println!("Completed database setup");
    Ok(())
}

async fn migrate_db(args: cli::DbMigrateArgs) -> Result<()> {
    let db_path = args.db_args.path;
    let conn_str = format!("sqlite://{}", db_path.to_string_lossy());

    if db_path.is_dir() {
        return Err(anyhow::anyhow!(
            "The specified path is a directory: {:?}",
            db_path
        ));
    }

    let db: DatabaseConnection = Database::connect(&conn_str)
        .await
        .context("Failed to connect to database")?;

    vault_migrations::Migrator::up(&db, None)
        .await
        .context("Failed to run database migrations")?;

    println!("Completed database migration");
    Ok(())
}
