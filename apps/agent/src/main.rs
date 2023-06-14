use config::{AgentConfig, ServerConfig};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::env;
use std::sync::Arc;
use std::{error::Error};

mod config;
mod stats;
mod server;

use stats::controller::SystemController;
use server::http::start_server;
use log::{info};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Initializing agent");

    let config = config::load_config();
    
    let ctl = Arc::new(Mutex::new(SystemController::new()));
    let sctl = ctl.clone();

    info!("Starting agent loop");
    tokio::spawn(agent_loop(ctl, config.get_agent().clone()));

    info!("Starting server loop");
    server_loop(sctl, config.get_server().clone()).await;

    Ok(())
}

async fn agent_loop(ctl: Arc<Mutex<SystemController>>, config: AgentConfig) {
    loop {
        let mut lo = ctl.lock().await;

        lo.refresh().await;

        drop(lo);

        sleep(Duration::from_millis(config.get_interval())).await;
    }
}

async fn server_loop(ctl: Arc<Mutex<SystemController>>, config: ServerConfig) {
    start_server(ctl, config).await;
}