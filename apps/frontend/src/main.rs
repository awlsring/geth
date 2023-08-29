use leptos::*;

use frontend::pages::App;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
