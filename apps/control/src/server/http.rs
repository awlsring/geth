use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::{
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    plugin::{PluginPipeline, IdentityPlugin},
    request::request_id::ServerRequestIdProviderLayer,
    AddExtensionLayer,
};

use log::{info, error};

use geth_control_server::{GethControl};
use geth_control_server::{input, output, error};

use crate::{config::ServerConfig, server::operation::server::get_server};

pub const DEFAULT_ADDRESS: &str = "0.0.0.0";

#[derive(Clone)]
struct Config;

pub struct State {
    // pub controller: Arc<Mutex<SystemController>>,
}

impl State {
    pub fn new() -> State {
        State {
        }
    }
}

pub async fn check_health(_input: input::HealthInput) -> Result<output::HealthOutput, error::HealthError> {
    Ok(output::HealthOutput { success: true })
}

pub async fn start_server(config: ServerConfig) {
    let plugins = PluginPipeline::new()
        // .print()
        // .auth(auth_controller.into(), Config)
        .insert_operation_extension()
        .instrument();

    let app = GethControl::builder_with_plugins(plugins, IdentityPlugin)
        .health(check_health)
        .get_server(get_server)
        .build()
        .expect("failed to build an instance of GethAgent");

    // create state to add to request
    let state = State::new();
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