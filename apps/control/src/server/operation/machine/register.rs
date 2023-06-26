use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::RegisterMachineOutput, input::RegisterMachineInput, error, model::{MachineSummary}};

use crate::server::http::State;

pub async fn register_machine(input: RegisterMachineInput, state: Extension<Arc<State>>) -> Result<RegisterMachineOutput, error::RegisterMachineError> {
    Err(error::RegisterMachineError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Machine {} not found", input.address()) }))
}