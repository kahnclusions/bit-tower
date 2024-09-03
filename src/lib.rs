pub mod app;
pub mod qbittorrent;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen()]
pub fn hydrate() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
