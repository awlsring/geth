use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::GetServerOutput, input::GetServerInput, error};

use crate::server::http::State;

pub async fn get_server(input: GetServerInput, state: Extension<Arc<State>>) -> Result<GetServerOutput, error::GetServerError> {
    Err(error::GetServerError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Server {} not found", input.id()) }))
}