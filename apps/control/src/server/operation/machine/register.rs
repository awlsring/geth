use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_control_server::{error, input::RegisterMachineInput, output::RegisterMachineOutput};
use log::{debug, info};

use crate::server::http::State;

use super::conversion::machine_to_summary;

pub async fn register_machine(
    input: RegisterMachineInput,
    state: Extension<Arc<State>>,
) -> Result<RegisterMachineOutput, error::RegisterMachineError> {
    info!(
        "Got register machine register. Registering at address {}",
        input.address()
    );
    debug!("Locking controller mutex");
    let mut controller = state.controller.lock().await;

    debug!("Registering machine via controller");
    let machine_result = controller
        .register_machine(input.address(), input.group_id())
        .await;

    match machine_result {
        Ok(m) => {
            debug!("Machine: {:?}", m);
            debug!("Converting machine to summary");
            let summary = machine_to_summary(m);
            debug!("Summary: {:?}", summary);
            Ok(RegisterMachineOutput { summary })
        }
        Err(e) => Err(error::RegisterMachineError::InvalidInputException(
            error::InvalidInputException {
                message: format!("Error making machine at address {}: {}", input.address(), e),
            },
        )),
    }
}
