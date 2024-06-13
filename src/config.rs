use std::path::Path;

use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,
    pub tokens: Vec<String>,
    pub port: u16,
    pub host: String,
    pub unix_socket: bool,
    pub unix_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "debug".to_owned(),
            tokens: vec![],
            port: 3000,
            host: "0.0.0.0".to_owned(),
            unix_socket: true,
            unix_path: "/var/run/docker.sock".to_owned(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./".to_owned());
        let config_path = Path::new(&config_path);
        let config: Result<Config> = Figment::from(Serialized::from(Config::default(), "default"))
            .merge(Toml::file(config_path.join("config.toml")))
            .merge(Json::file(config_path.join("config.json")))
            .merge(Yaml::file(config_path.join("config.yaml")))
            .merge(Env::prefixed("UPDATER_"))
            .extract();
        match config {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error loading configuration: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn log_level(&self) -> tracing::Level {
        match self.log_level.to_lowercase().as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        }
    }
}
