use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_agent_server::{output::GetSwapOutput, model::SwapSummary, input::GetSwapInput, error};

use crate::{server::server::State, stats::memory::Swap};


pub async fn get_swap(_input: GetSwapInput, state: Extension<Arc<State>>) -> Result<GetSwapOutput, error::GetSwapError> {
    let ctl = state.controller.lock().await;
    let swap = ctl.swap();

    let sum = swap_to_summary(swap);

    let output = GetSwapOutput {
        summary: sum
    };

    Ok(output)
}

pub fn swap_to_summary(swap: &Swap) -> SwapSummary {
    SwapSummary {
        total: *swap.total() as i64,
        available: *swap.available() as i64,
        used: *swap.used() as i64,
    }
}