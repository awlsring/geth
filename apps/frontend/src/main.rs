use leptos::*;

use frontend::pages::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    log!("Starting frontend");
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
