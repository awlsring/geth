mod config;
mod controller;
mod model;
mod persistence;
mod server;
mod service;

use config::ServerConfig;
use persistence::machine_repo::MachinePrismaRepository;
use prisma::PrismaClient;
use std::{env, error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::controller::agent::AgentController;
use dotenv::dotenv;
use log::info;
use server::http::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    info!("Initializing!");
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config = config::load_config();

    let prisma_conn = PrismaClient::_builder()
        .with_url(config.database().url().to_owned())
        .build()
        .await
        .unwrap();

    let prisma_conn_2 = PrismaClient::_builder()
        .with_url(config.database().url().to_owned())
        .build()
        .await
        .unwrap();

    let machine_repo = MachinePrismaRepository::new(prisma_conn);

    let agent_service = service::agent::AgentService::new();

    let controller = Arc::new(Mutex::new(AgentController::new(
        agent_service,
        machine_repo,
    )));
    let machine_repo = Arc::new(Mutex::new(
        persistence::machine_repo::MachinePrismaRepository::new(prisma_conn_2),
    ));

    info!("Starting server loop");
    server_loop(controller, machine_repo, config.get_server().clone()).await;

    Ok(())
}

async fn server_loop(
    controller: Arc<Mutex<AgentController>>,
    machine_repo: Arc<Mutex<MachinePrismaRepository>>,
    config: ServerConfig,
) {
    start_server(controller, config).await;
}
