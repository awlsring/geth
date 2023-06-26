use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::DescribeMachineUtilizationOutput, input::DescribeMachineUtilizationInput, error, model::{MachineSummary}};

use crate::server::http::State;

pub async fn describe_machine_utilization(input: DescribeMachineUtilizationInput, state: Extension<Arc<State>>) -> Result<DescribeMachineUtilizationOutput, error::DescribeMachineUtilizationError> {
    Err(error::DescribeMachineUtilizationError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Machine {} not found", input.id()) }))
}