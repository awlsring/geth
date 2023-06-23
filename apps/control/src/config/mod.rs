use std::{fs, env};
use serde::Deserialize;
use toml;
use log::{warn, debug};

#[derive(Debug, Deserialize)]
pub struct Config {
    server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                port: 8032,
                allowed_keys: vec![String::from("toes")],
                no_auth_operations: vec![String::from("Health")],
            },
        }
    }

}
impl Config {
    pub fn get_server(&self) -> &ServerConfig {
        &self.server
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    port: u16,
    allowed_keys: Vec<String>,
    no_auth_operations: Vec<String>,
}

impl ServerConfig {
    pub fn get_server_port(&self) -> u16 {
        self.port
    }

    pub fn allowed_keys(&self) -> &Vec<String> {
        &self.allowed_keys
    }

    pub fn no_auth_operations(&self) -> &Vec<String> {
        &self.no_auth_operations
    }
}

pub fn load_config() -> Config {
    let path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    debug!("Loading config from: {}", path);
    let config = fs::read_to_string(path);
    match config {
        Ok(config) => {
            debug!("Loaded config from file");
            parse_config(config)
        },
        Err(_) => {
            warn!("Failed to load config from file, using default config");
            Config::default()
        }
    }
}

fn parse_config(config: String) -> Config {
    let config = toml::from_str(&config);
    match config {
        Ok(config) => {
            debug!("Parsed config");
            return config;
        },
        Err(_) => {
            warn!("Failed to parse config, using default config");
            return Config::default();
        }
    }
}