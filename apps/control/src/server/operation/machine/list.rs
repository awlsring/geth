use std::{sync::Arc};

use aws_smithy_http_server::Extension;
use geth_control_server::{output::ListMachinesOutput, input::ListMachinesInput, error, model::{MachineSummary}};

use crate::server::http::State;

pub async fn list_machines(input: ListMachinesInput, state: Extension<Arc<State>>) -> Result<ListMachinesOutput, error::ListMachinesError> {
    Err(error::ListMachinesError::ValidationException(error::ValidationException { message: "meh".to_owned(), field_list: None }))
}