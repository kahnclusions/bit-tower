#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use json_patch::Patch;
use leptos::prelude::{
    arc_signal, ArcReadSignal, ArcRwSignal, Read, ReadSignal, RwSignal, Set, Update,
};
use qbittorrent_rs_proto::sync::{MainData, SyncMainDataFull};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[cfg(feature = "ssr")]
mod axum;
#[cfg(feature = "ssr")]
pub use axum::*;

/// Creates a signal which is controlled by the server.
///
/// This signal is initialized as T::default, is read-only on the client, and is updated through json patches
/// sent through a SSE connection.
///
/// # Example
///
/// ```
/// #[derive(Clone, Default, Serialize, Deserialize)]
/// pub struct Count {
///
///     pub value: i32,
/// }
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     // Create server signal
///     let count = create_sse_signal::<Count>("counter");
///
///     view! {
///         <h1>"Count: " {move || count().value.to_string()}</h1>
///     }
/// }
/// ```
#[allow(unused_variables)]
pub fn sse_sync_maindata(url: &str) -> ArcReadSignal<SyncMainDataFull> {
    let (get, set) = arc_signal(SyncMainDataFull::default());

    cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
    use std::{
        borrow::{BorrowMut, Cow},
        cell::RefCell,
        collections::HashMap,
        ops::{Deref, DerefMut},
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use cfg_if::cfg_if;
    use js_sys::{Function, JsString};
    use json_patch::Patch;
    use leptos::prelude::{create_effect, use_context, RwSignal};
    use qbittorrent_rs_proto::sync::MainData;
    use serde_json::Value;
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
    use web_sys::{EventSource, MessageEvent};

    // let mut delayed_updates: Arc<Mutex<Vec<Patch>>> = Arc::new(Mutex::new(Vec::new()));
    let es = EventSource::new(url).unwrap();

    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let ws_string = event
            .data()
            .dyn_into::<JsString>()
            .unwrap()
            .as_string()
            .unwrap();
        let result = serde_json::from_str::<MainData>(&ws_string);
        match result {
            Ok(MainData::Full(full_data)) => set.set(full_data),
            Ok(MainData::Partial(partial_data)) => {
                set.update(|doc| doc.apply_partial(partial_data))
            }
            Err(err) => {
                tracing::error!(error = ?err);
            }
        }
    }) as Box<dyn FnMut(_)>);
    let function: &Function = callback.as_ref().unchecked_ref();
    es.set_onmessage(Some(function));
    callback.forget();

        } else {
            tracing::error!(
                r#"server signal was used without a SSE being provided.

    Ensure you call `leptos_sse::provide_sse("http://localhost:3000/sse")` at the highest level in your app."#
            );
        }
        }

    get
}
