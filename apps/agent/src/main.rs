use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::{error::Error};

mod stats;

use stats::controller::SystemController;

mod server;
use server::server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Started!");

    let ctl = Arc::new(Mutex::new(SystemController::new()));
    let sctl = ctl.clone();

    println!("Starting agent loop!");
    tokio::spawn(agent_loop(ctl));

    println!("Starting server loop!");
    server_loop(sctl).await;

    Ok(())
}

async fn agent_loop(ctl: Arc<Mutex<SystemController>>) {
    loop {
        let mut lo = ctl.lock().await;

        lo.refresh().await;

        drop(lo);

        sleep(Duration::from_millis(5000)).await;
    }
}

async fn server_loop(ctl: Arc<Mutex<SystemController>>) {
    start_server(ctl).await;
}