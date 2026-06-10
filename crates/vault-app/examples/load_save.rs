use vault_app::AppError;

fn main() -> Result<(), AppError> {
    color_eyre::install()?;

    let config = vault_app::config::AppConfig::default();
    println!("Default config: {:#?}", config);

    vault_app::config::save_config(&config, Some("example_config"))?;
    println!("Config saved successfully.");

    let loaded_config = vault_app::config::load_config(Some("example_config"))?;
    println!("Loaded config: {:#?}", loaded_config);

    Ok(())
}
