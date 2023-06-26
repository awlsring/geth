use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::DescribeMachineOutput, input::DescribeMachineInput, error, model::{MachineSummary}};

use crate::server::http::State;

pub async fn describe_machine(input: DescribeMachineInput, state: Extension<Arc<State>>) -> Result<DescribeMachineOutput, error::DescribeMachineError> {
    Err(error::DescribeMachineError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Machine {} not found", input.id()) }))
}