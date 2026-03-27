use leptos::*;
mod api;
mod components;
pub mod pages;
mod app;
use crate::app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
