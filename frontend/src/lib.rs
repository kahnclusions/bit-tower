use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
