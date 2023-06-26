use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::CreateGroupOutput, input::CreateGroupInput, error};

use crate::server::http::State;

pub async fn create_group(input: CreateGroupInput, state: Extension<Arc<State>>) -> Result<CreateGroupOutput, error::CreateGroupError> {
    Err(error::CreateGroupError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Group {} not found", input.name()) }))
}