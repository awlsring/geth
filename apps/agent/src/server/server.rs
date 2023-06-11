use std::{net::SocketAddr, sync::Arc};

use aws_smithy_client::erase::{DynConnector, DynMiddleware};
use aws_smithy_http::endpoint::{SharedEndpointResolver, Endpoint};
use aws_smithy_http_server::{
    extension::OperationExtensionExt,
    instrumentation::InstrumentExt,
    plugin::{alb_health_check::AlbHealthCheckLayer, PluginPipeline},
    request::request_id::ServerRequestIdProviderLayer,
    AddExtensionLayer, Extension,
};

use aws_smithy_runtime::client::orchestrator::endpoints::{StaticUriEndpointResolver, StaticUriEndpointResolverParams};

use clap::Parser;
use tokio::sync::Mutex;
use crate::stats::controller::SystemController;

use super::plugin::PrintExt;

use hyper::{StatusCode, Uri};

use geth_agent_server::{GethAgent, model::SystemSummary};
use geth_agent_server::{input, output, error};
use geth_agent_server::input::{HealthInput};
use geth_agent_server::output::{HealthOutput};

mod operation;
use super::operation::overview::get_overview;
use super::operation::system::get_system;
use super::operation::memory::get_memory;
use super::operation::swap::get_swap;
use super::operation::disk::get_disk;
use super::operation::disk::list_disks;
use super::operation::network::get_network_interface;
use super::operation::network::list_network_interfaces;
use super::operation::cpu::get_cpu;
// async fn client() {
//     let smithy_client = Builder::new()
//         // Use the default HTTPS connector
//         .dyn_https_connector(Default::default())
//         // Use a no-op middleware
//         .middleware_fn(|request| request)
//         // Build a type-erased Smithy client
//         .build_dyn();

//     let c = SharedEndpointResolver::new("http://127.0.0.1:13734");
//     let config = geth_agent_client::Config::builder()
//         .endpoint_resolver(c)
//         .api_key(AuthApiKey::from("1234567890"))
//         .build();

//     let client = geth_agent_client::Client::with_config(smithy_client, config);

//     let resp = client.health().send().await.unwrap();
//     resp.success;
// }

// pub fn build_client() -> Client<DynConnector, DynMiddleware<DynConnector>> {
//     let smithy_client = Builder::new()
//         // Use the default HTTPS connector
//         .dyn_https_connector(Default::default())
//         // Use a no-op middleware
//         .middleware_fn(|request| request)
//         // Build a type-erased Smithy client
//         .build_dyn();


//     let e = SharedEndpointResolver::new("http://localhost:13734/");
//     let config = geth_agent_client::Config::builder()
//         .endpoint_resolver(e)
//         // .api_key(AuthApiKey::from("1234567890"))
//         .build();

//     let c = geth_agent_client::Client::with_config(smithy_client, config);
//     c
// }


pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
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

    let plugins = PluginPipeline::new()
        .print()
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