use leptos::prelude::*;

mod app;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(App);
}
