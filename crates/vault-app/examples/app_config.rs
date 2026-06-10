use vault_app::{AppError, config::AppConfig};

fn main() -> Result<(), AppError> {
    color_eyre::install()?;

    let config = AppConfig::new("db.sqlite");

    println!("Config {:#?}", config);

    let config_json =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");

    println!("\n=== Serialisation ===");
    println!("\nConfig (with db filepath) as JSON:\n{}", config_json);

    let deserialized_config_fp: AppConfig =
        serde_json::from_str(&config_json).expect("Failed to deserialize config from JSON");

    println!("\n=== Deserialisation ===");
    println!(
        "\nDeserialized Config (with db filepath): {:#?}",
        deserialized_config_fp
    );

    Ok(())
}
