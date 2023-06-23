use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::{
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    plugin::{PluginPipeline, IdentityPlugin},
    request::request_id::ServerRequestIdProviderLayer,
    AddExtensionLayer,
};

use log::{info, error};
use tokio::sync::Mutex;

use crate::{stats::controller::SystemController, config::ServerConfig, server::operation::{disk::{get_disk, list_disks}, container::{stream_container_logs, get_container, list_containers, stream_container_statistics}}};

use smithy_common::auth::controller::AuthController;
use smithy_common::auth::plugin::AuthExtension;
use smithy_common::print::plugin::PrintExt;

use geth_agent_server::{GethAgent};
use geth_agent_server::{input, output, error};

use super::operation::overview::get_overview;
use super::operation::system::get_system;
use super::operation::memory::get_memory;
use super::operation::volume::get_volume;
use super::operation::volume::list_volumes;
use super::operation::network::get_network_interface;
use super::operation::network::list_network_interfaces;
use super::operation::cpu::get_cpu;

pub const DEFAULT_ADDRESS: &str = "0.0.0.0";

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

pub async fn start_server(ctl: Arc<Mutex<SystemController>>, config: ServerConfig) {
    // TODO: Add config where keys can be stored and retrived
    let auth_controller = AuthController::new(config.no_auth_operations(), config.allowed_keys());

    let plugins = PluginPipeline::new()
        .print()
        .auth(auth_controller.into(), Config)
        .insert_operation_extension()
        .instrument();

    let app = GethAgent::builder_with_plugins(plugins, IdentityPlugin)
        .health(check_health)
        .get_overview(get_overview)
        .get_system(get_system)
        .get_memory(get_memory)
        .get_cpu(get_cpu)
        .get_disk(get_disk)
        .list_disks(list_disks)
        .get_volume(get_volume)
        .list_volumes(list_volumes)
        .get_network_interface(get_network_interface)
        .list_network_interfaces(list_network_interfaces)
        .stream_container_logs(stream_container_logs)
        .stream_container_statistics(stream_container_statistics)
        .get_container(get_container)
        .list_containers(list_containers)
        .build()
        .expect("failed to build an instance of GethAgent");

    // create state to add to request
    let state = State::new(ctl);
    let app = app
        .layer(&AddExtensionLayer::new(Arc::new(state)))
        .layer(&ServerRequestIdProviderLayer::new());

    let make_app = app.into_make_service_with_connect_info::<SocketAddr>();

    info!("Starting server on: {}:{}", DEFAULT_ADDRESS, config.get_server_port());
    let bind: SocketAddr = format!("{}:{}", DEFAULT_ADDRESS, config.get_server_port())
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    // Run forever-ish...
    if let Err(err) = server.await {
        error!("server error: {}", err);
    }
}