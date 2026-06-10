use thiserror::Error;

pub mod cli;
pub mod config;
pub mod tui;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("TUI error: {0}")]
    Tui(#[from] tui::TuiError),
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("ColorEyre error: {0}")]
    ColorEyre(#[from] color_eyre::eyre::Report),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error("The specified database path is a directory: {0}")]
    DbPathIsDirectory(String),
}
