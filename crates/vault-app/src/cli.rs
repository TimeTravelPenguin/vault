use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// CLI for the vault application.
#[derive(Debug, Parser)]
#[command(name = "vault", version, about, long_about = None)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands for the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the TUI.
    Tui(TuiArgs),
    /// Database management commands.
    #[command(subcommand)]
    Db(DbSubcommand),
}

#[derive(Debug, Parser)]
pub struct TuiArgs {
    #[command(flatten)]
    pub db_args: DbArgs,
}

#[derive(Debug, Subcommand)]
pub enum DbSubcommand {
    /// Create a new database.
    Create(DbCreateArgs),
    /// Run database migrations.
    Migrate(DbMigrateArgs),
}

#[derive(Debug, Parser)]
pub struct DbCreateArgs {
    #[command(flatten)]
    pub db_args: DbArgs,

    /// Whether to replace the existing database file if it exists.
    #[arg(long)]
    pub replace: bool,
}

#[derive(Debug, Parser)]
pub struct DbMigrateArgs {
    #[command(flatten)]
    pub db_args: DbArgs,
}

#[derive(Debug, Args)]
pub struct DbArgs {
    /// The path to the database file.
    #[arg(default_value = "db.sqlite")]
    pub path: PathBuf,
}
