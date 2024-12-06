use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
pub struct Cli {
    /// Path to the configuration file containing the URLs in JSON
    #[arg(short, long, default_value = "url_list.json")]
    pub config: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub urls: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_config_deserialization() {
        let json_data = r#"{ "urls": ["https://rust-lang.org"] }"#;
        let config: Config = serde_json::from_str(json_data).expect("Failed to deserialize");
        assert_eq!(config.urls.len(), 1);
        assert_eq!(config.urls[0], "https://rust-lang.org");
    }

    #[test]
    fn test_cli_default_value() {
        let args = Cli::parse_from(&["test-binary"]);
        assert_eq!(args.config, "url_list.json");
    }
}