use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::DeleteGroupOutput, input::DeleteGroupInput, error};

use crate::server::http::State;

pub async fn delete_group(input: DeleteGroupInput, state: Extension<Arc<State>>) -> Result<DeleteGroupOutput, error::DeleteGroupError> {
    Err(error::DeleteGroupError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Group {} not found", input.id()) }))
}