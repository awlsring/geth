use geth_control_client::types::{CpuSummary, MachineStatus, MachineStatusSummary, MachineSummary};
use leptos::*;

use crate::components::machine::{
    status_icon::StatusIcon,
    summary_format::{format_arch, format_cpu, format_disk, format_mem},
};

fn format_status(status: Option<&MachineStatusSummary>) -> String {
    let unknown = "UNKNOWN".to_string();
    match status {
        Some(status) => match status.status() {
            Some(MachineStatus::Running) => "RUNNING".to_string(),
            Some(MachineStatus::Stopped) => "STOPPED".to_string(),
            _ => unknown,
        },
        _ => unknown,
    }
}

#[component]
fn GroupDetail<'a>(
    cx: Scope,
    key: &'a str,
    value: Option<&'a str>,
    include_break: bool,
) -> impl IntoView {
    match value {
        Some(value) => view! { cx,
            <div class="flex">
                <p class="font-semibold pr-1">{ format!("{}:", key) }</p>
                <p class="">{ value.to_string() }</p>
                { if include_break { view! { cx, <div class="h-5 border-l border-gray-30 mx-2"></div> } } else { view! { cx, <div></div> } } }
            </div>
        },
        _ => view! { cx,
            <div></div>
        },
    }
}

#[component]
pub fn MachineOverview(cx: Scope, summary: MachineSummary) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col border border-gray-200 text-gray-600 bg-gray-100 px-4 my-2 rounded-lg">
            <div class="flex justify-between">
                <div class="flex items-center">
                    <StatusIcon status={ summary.status() } />
                    <p class="px-2 mt-3 text-gray-700">{ format_status(summary.status()) }</p>
                </div>
                <div class="flex items-center">
                    <p class="px-2 mt-3 text-gray-700">{ "Action" }</p>
                    <p class="px-2 mt-3 text-gray-700">{ "Action2" }</p>
                </div>
            </div>
            <div class="border-t mt-2 border-gray-200"></div>
            <div class="flex flex-row justify-between">
                <div class="col-span-4 px-2 py-2 flex flex-col">
                    <div class="flex flex-col text-large font-semibold">
                        <div class="flex justify-between">
                            <p class="text-gray-600">{ "Summary"}</p>
                        </div>
                        <div class="flex flex-col text-sm">
                            <div class="flex items-center">
                                <p> { format_cpu(summary.cpu()) } </p>
                                <p class="px-2 text-xs text-gray-500"> { format!("({})", format_arch(summary.cpu())) } </p>
                            </div>
                            <div class="flex items-center">
                                <p> { format_mem(summary.memory()) } </p>
                                <p class="px-2"> { format_disk(summary.storage()) } </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="border-t border-gray-200"></div>
            <div class="flex py-2 text-sm text-gray-500">
                <GroupDetail key="Group" value={ summary.group() } include_break=true />
                <GroupDetail key="Location" value={ summary.location() } include_break=true />
                <GroupDetail key="Type" value={ summary.provider_type() } include_break=true />
                <GroupDetail key="Local ID" value={ summary.provider_id() } include_break=true />
                <GroupDetail key="Agent" value={ Some("false") } include_break=false />
            </div>
        </div>
    }
}
