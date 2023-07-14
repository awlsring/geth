use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_control_server::{error, input::RemoveMachineInput, output::RemoveMachineOutput};
use log::error;

use crate::server::http::State;

pub async fn remove_machine(
    input: RemoveMachineInput,
    state: Extension<Arc<State>>,
) -> Result<RemoveMachineOutput, error::RemoveMachineError> {
    let mut controller = state.controller.lock().await;

    let machine_result = controller.remove_machine(input.id()).await;

    match machine_result {
        Ok(_) => Ok(RemoveMachineOutput { success: true }),
        Err(e) => {
            error!("Error removing {}: {}", input.id(), e);
            Err(error::RemoveMachineError::ResourceNotFoundException(
                error::ResourceNotFoundException {
                    message: format!("No machine found with id {}", input.id()),
                },
            ))
        }
    }
}
