use vault_app::{AppError, config};

fn main() -> Result<(), AppError> {
    color_eyre::install()?;

    let config_path = config::get_config_path(Some("example_config"))?;
    if config_path.exists() {
        panic!(
            "Config file already exists at {:?}. Aborting to prevent overwriting.",
            config_path
        );
    }

    let config = vault_app::config::AppConfig::default();
    println!("Default config: {:#?}", config);

    vault_app::config::save_config(&config, Some("example_config"))?;
    println!("\nConfig saved successfully.");

    let loaded_config = vault_app::config::load_config(Some("example_config"))?;
    println!("\nLoaded config: {:#?}", loaded_config);

    std::fs::remove_file(config_path)?;
    Ok(())
}
