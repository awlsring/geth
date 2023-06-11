use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_agent_server::{input::GetSystemInput, output::GetSystemOutput, model::SystemSummary, error};

use crate::{server::server::State, stats::system::System};


pub async fn get_system(_input: GetSystemInput, state: Extension<Arc<State>>) -> Result<GetSystemOutput, error::GetSystemError> {
    let ctl = state.controller.lock().await;
    let sys = ctl.system();

    let sum = system_to_summary(&sys);

    let output = GetSystemOutput {
        summary: sum
    };

    Ok(output)
}

pub fn system_to_summary(system: &System) -> SystemSummary {
    let fam_opt = system.family().to_owned();
    let kernel = system.kernel_version().to_owned();
    let os = system.os().to_owned();
    let os_version = system.os_version().to_owned();
    let os_pretty = system.os_pretty().to_owned();
    let hostname = system.hostname().to_owned();
    let boot_time = system.boot_time().to_owned();
    let up_time = system.up_time().to_owned();

    SystemSummary {
        family: fam_opt,
        kernel_version: kernel,
        os: os,
        os_version: os_version,
        os_pretty: os_pretty,
        hostname: hostname,
        boot_time: boot_time as i64,
        up_time: up_time as i64,
    }
}