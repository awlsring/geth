use geth_control_client::types::MachineSummary;
use leptos::*;
use leptos_router::Outlet;

use crate::components::machine::details::overview::MachineOverview;

#[component]
pub fn MachineDetailView(cx: Scope, summary: MachineSummary) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col justify-between rounded">
            <MachineOverview summary=summary/>
            <div class="flex my-2 bg-gray-100 ">
                <Outlet/>
            </div>
        </div>
    }
}
