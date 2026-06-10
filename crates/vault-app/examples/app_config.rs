use vault_app::{
    AppError,
    config::{AppConfig, from_file_path, try_from_connection_string},
};

fn main() -> Result<(), AppError> {
    color_eyre::install()?;

    let config_fp = AppConfig::new(from_file_path("db.sqlite"));

    let config_cs = AppConfig::new(
        try_from_connection_string("sqlite://db.sqlite?mode=rwc")
            .expect("Failed to parse database connection string"),
    );

    println!("Config (with db filepath): {:#?}", config_fp);
    println!("\nConfig (with db connection string): {:#?}", config_cs);

    let config_fp_json =
        serde_json::to_string_pretty(&config_fp).expect("Failed to serialize config to JSON");
    let config_cs_json =
        serde_json::to_string_pretty(&config_cs).expect("Failed to serialize config to JSON");

    println!("\n=== Serialisation ===");
    println!("\nConfig (with db filepath) as JSON:\n{}", config_fp_json);
    println!(
        "\nConfig (with db connection string) as JSON:\n{}",
        config_cs_json
    );

    let deserialized_config_fp: AppConfig =
        serde_json::from_str(&config_fp_json).expect("Failed to deserialize config from JSON");
    let deserialized_config_cs: AppConfig =
        serde_json::from_str(&config_cs_json).expect("Failed to deserialize config from JSON");

    println!("\n=== Deserialisation ===");
    println!(
        "\nDeserialized Config (with db filepath): {:#?}",
        deserialized_config_fp
    );
    println!(
        "\nDeserialized Config (with db connection string): {:#?}",
        deserialized_config_cs
    );

    Ok(())
}
