mod server;
mod config;

use config::{ServerConfig, Config};
use std::{env, error::Error};

use server::http::start_server;
use log::{info, debug, error};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    info!("Initializing!");
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config = config::load_config();

    info!("Starting server loop");
    server_loop(config.get_server().clone()).await;

    Ok(())
}

async fn server_loop(config: ServerConfig) {
    start_server(config).await;
}