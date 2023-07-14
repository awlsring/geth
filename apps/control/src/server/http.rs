use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::{
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    plugin::{IdentityPlugin, PluginPipeline},
    request::request_id::ServerRequestIdProviderLayer,
    AddExtensionLayer,
};

use log::{error, info};

use geth_control_server::GethControl;
use geth_control_server::{error, input, output};
use smithy_common::auth::controller::AuthController;
use smithy_common::auth::plugin::AuthExtension;
use smithy_common::print::plugin::PrintExt;
use tokio::sync::Mutex;

use crate::{
    config::ServerConfig,
    controller::agent::AgentController,
    server::operation::{
        group::{
            create::create_group, delete::delete_group, describe::describe_group, list::list_groups,
        },
        machine::{
            describe::describe_machine, list::list_machines, register::register_machine,
            remove::remove_machine, utilization::describe_machine_utilization,
        },
    },
};

pub const DEFAULT_ADDRESS: &str = "0.0.0.0";

#[derive(Clone)]
struct Config;

pub struct State {
    pub controller: Arc<Mutex<AgentController>>,
}

impl State {
    pub fn new(controller: Arc<Mutex<AgentController>>) -> State {
        State { controller }
    }
}

pub async fn check_health(
    _input: input::HealthInput,
) -> Result<output::HealthOutput, error::HealthError> {
    Ok(output::HealthOutput { success: true })
}

pub async fn start_server(controller: Arc<Mutex<AgentController>>, config: ServerConfig) {
    let auth_controller = AuthController::new(config.no_auth_operations(), config.allowed_keys());

    let plugins = PluginPipeline::new()
        .print()
        .auth(auth_controller.into(), Config)
        .insert_operation_extension()
        .instrument();

    let app = GethControl::builder_with_plugins(plugins, IdentityPlugin)
        .health(check_health)
        .describe_machine(describe_machine)
        .describe_machine_utilization(describe_machine_utilization)
        .list_machines(list_machines)
        .register_machine(register_machine)
        .remove_machine(remove_machine)
        .create_group(create_group)
        .delete_group(delete_group)
        .describe_group(describe_group)
        .list_groups(list_groups)
        .build()
        .expect("failed to build an instance of Geth Control Server");

    // create state to add to request
    let state = State::new(controller);
    let app = app
        .layer(&AddExtensionLayer::new(Arc::new(state)))
        .layer(&ServerRequestIdProviderLayer::new());

    let make_app = app.into_make_service_with_connect_info::<SocketAddr>();

    info!(
        "Starting server on: {}:{}",
        DEFAULT_ADDRESS,
        config.get_server_port()
    );
    let bind: SocketAddr = format!("{}:{}", DEFAULT_ADDRESS, config.get_server_port())
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    // Run forever-ish...
    if let Err(err) = server.await {
        error!("server error: {}", err);
    }
}
