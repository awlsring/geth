use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::ListGroupsOutput, input::ListGroupsInput, error};

use crate::server::http::State;

pub async fn list_groups(input: ListGroupsInput, state: Extension<Arc<State>>) -> Result<ListGroupsOutput, error::ListGroupsError> {
    Err(error::ListGroupsError::ValidationException(error::ValidationException { message: format!("Machine not found"), field_list: None }))
}