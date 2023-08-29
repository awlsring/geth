use leptos::*;

use crate::components::machine::header::Header;

#[component]
pub fn Machines(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container text-center">
            <Header />
            // { display_machines(machines) }
        </div>
    }
}
