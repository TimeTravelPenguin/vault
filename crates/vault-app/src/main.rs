use clap::Parser;

use vault_app::{AppError, cli, config, db::VaultStore};

type Result<T> = std::result::Result<T, AppError>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Tui(args) => {
            run_tui(args).await?;
        }
    };

    Ok(())
}

async fn run_tui(_args: cli::TuiArgs) -> Result<()> {
    color_eyre::install()?;

    let config = config::load_config(Some(config::CONFIG_FILE_NAME))?;
    let store = VaultStore::new(config.database.clone()).await?;

    vault_app::tui::run(config, store).await?;

    Ok(())
}
