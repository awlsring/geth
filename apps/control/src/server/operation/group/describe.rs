use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::DescribeGroupOutput, input::DescribeGroupInput, error};

use crate::server::http::State;

pub async fn describe_group(input: DescribeGroupInput, state: Extension<Arc<State>>) -> Result<DescribeGroupOutput, error::DescribeGroupError> {
    Err(error::DescribeGroupError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Group {} not found", input.id()) }))
}