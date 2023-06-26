use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::RemoveMachineOutput, input::RemoveMachineInput, error, model::{MachineSummary}};

use crate::server::http::State;

pub async fn remove_machine(input: RemoveMachineInput, state: Extension<Arc<State>>) -> Result<RemoveMachineOutput, error::RemoveMachineError> {
    Err(error::RemoveMachineError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Machine {} not found", input.id()) }))
}