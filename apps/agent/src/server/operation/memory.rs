use std::sync::Arc;

use aws_smithy_http_server::Extension;
use geth_agent_server::{output::GetMemoryOutput, model::MemorySummary, input::GetMemoryInput, error};

use crate::{server::http::State, stats::memory::Memory};


pub async fn get_memory(_input: GetMemoryInput, state: Extension<Arc<State>>) -> Result<GetMemoryOutput, error::GetMemoryError> {
    let ctl = state.controller.lock().await;
    let mem = ctl.memory();

    let sum = memory_to_summary(mem);

    let output = GetMemoryOutput {
        summary: sum
    };

    Ok(output)
}

pub fn memory_to_summary(memory: &Memory) -> MemorySummary {
    MemorySummary {
        total: *memory.total() as i64,
        available: *memory.available() as i64,
        used: *memory.used() as i64,
    }
}