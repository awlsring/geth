use leptos::*;
use leptos_icons::*;

#[component]
pub fn DetailHeader(cx: Scope, id: String) -> impl IntoView {
    view! { cx,
        <header class="flex items-center justify-between px-4">
            <div class="flex items-center">
                <a href={ "/machines" }>
                    <h1 class="text-2xl text-gray-600 hover:text-emerald-800">{ "Machines" }</h1>
                </a>
                <h1 class="text-xl text-gray-600 px-1">{ "/" }</h1>
                <h1 class="text-large font-semibold pt-1 pr-1">{ id }</h1>
            </div>
            <div class="flex space-x-4 p-1">
                <div class="p-1">
                    <select class="block w-full py-2 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-gray-700">
                        <option value="1">{"choice"}</option>
                        <option value="2">{"choice"}</option>
                        <option value="3">{"choice"}</option>
                        <option value="*">{"choice"}</option>
                    </select>
                </div>
                <div class="flex mx-2 p-1">
                    <button>
                        <Icon icon=icon!(BsCardList) width="1.5em" height="1.5em" class="text-gray-600 hover:text-gray-400" />
                    </button>
                    <div class="px-2"></div>
                    <button class="">
                        <Icon icon=icon!(BsCardText) width="1.5em" height="1.5em" class="text-gray-600 hover:text-gray-400" />
                    </button>
                </div>
            </div>
        </header>
    }
}
