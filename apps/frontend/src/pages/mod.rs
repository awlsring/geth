use leptos::*;
use leptos_router::*;

use crate::{
    components::nav::Nav,
    pages::{home::Home, machine_detail::MachineDetail, machines::Machines},
};

pub mod home;
pub mod machine_detail;
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
                        <Route path="machines/:id" view=MachineDetail>
                            <Route path="" view=|cx| view! { cx,
                                <div class="select-user">
                                    "option 1"
                                </div>
                            }/>
                            <Route path="option1" view=|cx| view! { cx,
                                <div class="select-user">
                                    "option 1"
                                </div>
                            }/>
                            <Route path="option2" view=|cx| view! { cx,
                                <div class="select-user">
                                    "option 2"
                                </div>
                            }/>
                        </Route>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
