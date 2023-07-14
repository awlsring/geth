use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_control_server::{error, input::DescribeMachineInput, output::DescribeMachineOutput};

use crate::server::http::State;

use super::conversion::machine_to_summary;

pub async fn describe_machine(
    input: DescribeMachineInput,
    state: Extension<Arc<State>>,
) -> Result<DescribeMachineOutput, error::DescribeMachineError> {
    let mut controller = state.controller.lock().await;

    let machine_result = controller.get_machine_summary(input.id()).await;

    match machine_result {
        Ok(m) => {
            let summary = machine_to_summary(m);
            Ok(DescribeMachineOutput { summary })
        }
        Err(e) => Err(error::DescribeMachineError::ResourceNotFoundException(
            error::ResourceNotFoundException {
                message: format!("Error describing machine at address {}: {}", input.id(), e),
            },
        )),
    }
}
