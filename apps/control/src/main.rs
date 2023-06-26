mod server;
mod config;
mod controller;

use config::{ServerConfig, Config};
use std::{env, error::Error, sync::{Arc}};
use tokio::sync::Mutex;

use server::http::start_server;
use log::{info, debug, error};
use env_logger;

use crate::controller::agent::AgentController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    info!("Initializing!");
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config = config::load_config();

    let controller = Arc::new(Mutex::new(AgentController::new()));

    info!("Starting server loop");
    server_loop(controller, config.get_server().clone()).await;

    Ok(())
}

async fn server_loop(controller: Arc<Mutex<AgentController>>, config: ServerConfig) {
    start_server(controller, config).await;
}