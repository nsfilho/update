use std::{collections::HashMap, path::Path};

use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ConfigRegistry {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

/// # Configuration for the application
///
/// Permit to configure the application with the following options:
/// * log_level: The level of logging - default: debug
/// * tokens: A list of tokens to be used for authentication
/// * port: The port to run the server on - default: 3000
/// * host: The host to run the server on - default: 0.0.0.0
/// * docker_url: The url to the docker daemon - default: http://localhost:8080
/// * registries: A list of docker registries to authenticate with
///
/// You can defined the path for config files via env: `CONFIG_PATH`.
/// The default path is the `cwd`.
///
/// You can configure the application using the following methods:
/// * config.toml
/// * config.json
/// * config.yaml
/// * Environment variables prefixed with UPDATER_
///
/// ## Example: config.json
///
/// ```json
/// {
///    "log_level": "info",
///    "tokens": {
///      "github": "secret"
///    },
///    "port": 3000,
///    "host": "0.0.0.0",
///    "docker_url": "http://localhost:8080",
///         {
///             "name": "usign",
///             "url": "http://registry.usign.io",
///             "username": "servers",
///             "password": "secret"
///         }
///    ]
///    "graceful_shutdown_timeout": 30,
///    "http_body_limit": 1024,
///    "http_request_timeout": 10
/// }
///
/// ## Parameters
///
/// * graceful_shutdown_timeout: The time to wait for a graceful shutdown - default: 30 seconds
/// * http_body_limit: The maximum size of the request body - default: 1KB
/// * http_request_timeout: The timeout for a request - default: 10 seconds
///
/// ```
///
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub log_level: String,
    pub tokens: HashMap<String, String>,
    pub port: u16,
    pub host: String,
    pub docker_url: String,
    pub registries: Vec<ConfigRegistry>,
    pub graceful_shutdown_timeout: u64,
    pub http_body_limit: usize,
    pub http_request_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "debug".to_owned(),
            tokens: HashMap::new(),
            port: 3000,
            host: "0.0.0.0".to_owned(),
            docker_url: "http://localhost:8080".to_owned(),
            registries: vec![],
            graceful_shutdown_timeout: 30,
            http_body_limit: 1024,
            http_request_timeout: 10,
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
