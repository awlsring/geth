use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_control_server::{
    error, input::ListMachinesInput, model::MachineSummary, output::ListMachinesOutput,
};

use crate::server::http::State;

pub async fn list_machines(
    input: ListMachinesInput,
    state: Extension<Arc<State>>,
) -> Result<ListMachinesOutput, error::ListMachinesError> {
    let mut controller = state.controller.lock().await;

    // let machines_result = controller.list_machines().await;

    Err(error::ListMachinesError::ValidationException(
        error::ValidationException {
            message: "meh".to_owned(),
            field_list: None,
        },
    ))
}
