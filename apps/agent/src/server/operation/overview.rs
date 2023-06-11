use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_agent_server::{output::GetOverviewOutput, model::OverviewSummary, input::GetOverviewInput, error};

use crate::server::server::State;

use super::{system::system_to_summary, swap::swap_to_summary, disk::disks_to_summaries, memory::memory_to_summary, cpu::cpu_to_summary, network::network_interfaces_to_summaries};


pub async fn get_overview(_input: GetOverviewInput, state: Extension<Arc<State>>) -> Result<GetOverviewOutput, error::GetOverviewError> {
    let ctl = state.controller.lock().await;
    let network = ctl.network();
    let cpu = ctl.cpu();
    let disks = ctl.storage();
    let mem = ctl.memory();
    let swap = ctl.swap();
    let sys = ctl.system();

    let network = network_interfaces_to_summaries(network.network_interfaces());
    let cpu = cpu_to_summary(cpu);
    let memory = memory_to_summary(mem);
    let swap = swap_to_summary(swap);
    let system = system_to_summary(sys);
    let disk = disks_to_summaries(disks.disks());

    let sum = OverviewSummary {
        network,
        cpu,
        memory,
        swap,
        system,
        disk,
    };

    let output = GetOverviewOutput {
        summary: sum
    };

    Ok(output)
}