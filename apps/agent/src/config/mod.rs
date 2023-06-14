use std::{fs, env};
use serde::Deserialize;
use toml;
use log::{warn, debug};

#[derive(Debug, Deserialize)]
pub struct Config {
    agent: AgentConfig,
    server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            agent: AgentConfig {
                interval: 10,
                cpu: None,
                mem: None,
                disk: None,
                network: None,
            },
            server: ServerConfig {
                port: 7032,
            },
        }
    }

}
impl Config {
    pub fn get_server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn get_agent(&self) -> &AgentConfig {
        &self.agent
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    port: u16,
}

impl ServerConfig {
    pub fn get_server_port(&self) -> u16 {
        self.port
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AgentConfig {
    interval: u64,
    cpu: Option<u64>,
    mem: Option<u64>,
    disk: Option<u64>,
    network: Option<u64>,
}

impl AgentConfig {
    pub fn get_interval(&self) -> u64 {
        self.interval
    }
    pub fn get_cpu_interval(&self) -> Option<u64> {
        self.cpu
    }
    pub fn get_mem_interval(&self) -> Option<u64> {
        self.mem
    }
    pub fn get_disk_interval(&self) -> Option<u64> {
        self.disk
    }
    pub fn get_network_interval(&self) -> Option<u64> {
        self.network
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