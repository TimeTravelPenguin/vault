use clap::Parser;
use sea_orm::{Database, DatabaseConnection};

use vault_app::{AppError, cli};
use vault_migrations::MigratorTrait;

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
    };

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
