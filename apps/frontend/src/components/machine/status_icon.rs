use geth_control_client::types::{MachineStatus, MachineStatusSummary};
use leptos::*;
use leptos_icons::*;

#[component]
pub fn StatusIcon<'a>(cx: Scope, status: Option<&'a MachineStatusSummary>) -> impl IntoView {
    match status {
        Some(status) => match status.status() {
            Some(MachineStatus::Running) => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-green-500 mt-3" />
            },
            Some(MachineStatus::Stopped) => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-red-500 mt-3" />
            },
            _ => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-gray-500 mt-3" />
            },
        },
        _ => view! { cx,
            <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-gray-500 mt-3" />
        },
    }
}
