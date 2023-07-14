use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_control_server::{error, input::ListMachinesInput, output::ListMachinesOutput};

use crate::server::http::State;

use super::conversion::machine_to_summary;

pub async fn list_machines(
    _: ListMachinesInput,
    state: Extension<Arc<State>>,
) -> Result<ListMachinesOutput, error::ListMachinesError> {
    let mut controller = state.controller.lock().await;

    let machines_result = controller.list_machines().await;

    let mut summaries = Vec::new();
    for machine in machines_result.iter() {
        summaries.push(machine_to_summary(machine.clone()))
    }

    Ok(ListMachinesOutput { summaries })
}
