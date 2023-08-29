use leptos::*;
use leptos_router::*;

use crate::{
    components::nav::Nav,
    pages::{home::Home, machines::Machines},
};

pub mod home;
pub mod machines;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <div class="flex min-h-screen flex-col">
                <Nav/>
                <div class="ml-36">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/machines" view=Machines/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
