use leptos::*;
use leptos_icons::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container py-4">
            <nav class="fixed left-0 top-0 h-full bg-gray-800 text-white w-30">
                <div class="flex flex-col p-4 text-left">
                    <a href="/">
                        <img class="w-20 h-20 mx-auto" src="logo.svg"/>
                    </a>
                    <a href="/machines" class="text-emerald-600 hover:text-emerald-500">
                        <div class="flex items-center py-2">
                            <Icon icon=icon!(FaServerSolid) />
                            <p class="ml-2">{ "Machines" }</p>
                        </div>
                    </a>
                    <a href="/storage" class="text-emerald-600 hover:text-emerald-500">
                        <div class="flex items-center py-2">
                            <Icon icon=icon!(FaBucketSolid) />
                            <p class="ml-2">{ "Storage" }</p>
                        </div>
                    </a>
                    <a href="/networks" class="text-emerald-600 hover:text-emerald-500">
                        <div class="flex items-center py-2">
                            <Icon icon=icon!(BiNetworkChartSolid) />
                            <p class="ml-2">{ "Networks" }</p>
                        </div>
                    </a>
                    <a href="/automation" class="text-emerald-600 hover:text-emerald-500">
                        <div class="flex items-center py-2">
                            <Icon icon=icon!(FaRobotSolid) />
                            <p class="ml-2">{ "Automation" }</p>
                        </div>
                    </a>
                    <a href="/settings" class="text-emerald-600 hover:text-emerald-500">
                        <div class="flex items-center py-2">
                            <Icon icon=icon!(FaGearSolid) />
                            <p class="ml-2">{ "Settings" }</p>
                        </div>
                    </a>
                </div>
            </nav>
        </div>
    }
}
