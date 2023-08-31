use geth_control_client::types::MachineSummary;
use leptos::*;

#[component]
pub fn MachineOverview(cx: Scope, summary: MachineSummary) -> impl IntoView {
    view! { cx,
        <div class="flex items-center justify-between bg-gray-100 px-4 my-5 rounded">
            <div class="flex items-center">
                <div class="flex flex-col">
                    <div class="text-gray-500 text-sm">"ID"</div>
                    <div class="text-gray-900 text-lg">{summary.identifier().unwrap_or("").to_string()}</div>
                </div>
                <div class="flex flex-col ml-5">
                    <div class="text-gray-500 text-sm">"Class"</div>
                    <div class="text-gray-900 text-lg">{ "VM "}</div>
                </div>
                <div class="flex flex-col ml-5">
                    <div class="text-gray-500 text-sm">"Location"</div>
                    <div class="text-gray-900 text-lg">{ "us-east-1 "}</div>
                </div>
            </div>
            <div class="flex items-center">
                <div class="flex flex-col">
                    <div class="text-gray-500 text-sm">"Status"</div>
                    <div class="text-gray-900 text-lg">{ "running" }</div>
                </div>
                <div class="flex flex-col ml-5">
                    <div class="text-gray-500 text-sm">"Last Checked"</div>
                    <div class="text-gray-900 text-lg">{ "now" }</div>
                </div>
            </div>
        </div>
    }
}
