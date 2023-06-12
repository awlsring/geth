use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::{
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    plugin::{alb_health_check::AlbHealthCheckLayer, PluginPipeline},
    request::request_id::ServerRequestIdProviderLayer,
    AddExtensionLayer,
};

use clap::Parser;
use tokio::sync::Mutex;

use crate::stats::controller::SystemController;

use super::{plugin::PrintExt, auth::{controller::AuthController}};

use super::auth::plugin::AuthExtension;

use hyper::{StatusCode};

use geth_agent_server::{GethAgent};
use geth_agent_server::{input, output, error};

use super::operation::overview::get_overview;
use super::operation::system::get_system;
use super::operation::memory::get_memory;
use super::operation::swap::get_swap;
use super::operation::disk::get_disk;
use super::operation::disk::list_disks;
use super::operation::network::get_network_interface;
use super::operation::network::list_network_interfaces;
use super::operation::cpu::get_cpu;



pub const DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 13734;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyper server bind address.
    #[clap(short, long, action, default_value = DEFAULT_ADDRESS)]
    address: String,
    /// Hyper server bind port.
    #[clap(short, long, action, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Clone)]
struct Config;

pub struct State {
    pub controller: Arc<Mutex<SystemController>>,
}

impl State {
    pub fn new(ctl: Arc<Mutex<SystemController>>) -> State {
        State {
            controller: ctl,
        }
    }
}

pub async fn check_health(_input: input::HealthInput) -> Result<output::HealthOutput, error::HealthError> {
    Ok(output::HealthOutput { success: true })
}

pub async fn start_server(ctl: Arc<Mutex<SystemController>>) {
    let args = Args::parse();

    // TODO: Add config where keys can be stored and retrived
    let auth_controller = AuthController::new();

    let plugins = PluginPipeline::new()
        .print()
        .auth(auth_controller.into(), Config)
        .insert_operation_extension()
        .instrument()
        .http_layer(AlbHealthCheckLayer::from_handler("/ping", |_req| async {
            StatusCode::OK
        }));

    let app = GethAgent::builder_with_plugins(plugins)
        .health(check_health)
        .get_overview(get_overview)
        .get_system(get_system)
        .get_memory(get_memory)
        .get_swap(get_swap)
        .get_cpu(get_cpu)
        .get_disk(get_disk)
        .list_disks(list_disks)
        .get_network_interface(get_network_interface)
        .list_network_interfaces(list_network_interfaces)
        .build()
        .expect("failed to build an instance of GethAgent");

    // create state to add to request
    let state = State::new(ctl);
    let app = app
        .layer(&AddExtensionLayer::new(Arc::new(state)))
        .layer(&ServerRequestIdProviderLayer::new());

    let make_app = app.into_make_service_with_connect_info::<SocketAddr>();

    let bind: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    // Run forever-ish...
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}