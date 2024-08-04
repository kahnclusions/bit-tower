#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use js_sys::Array;
use json_patch::Patch;
use leptos::prelude::*;
use logging::console_log;
use qbittorrent_rs_proto::sync::{MainData, SyncMainDataFull};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use signals::SyncState;
use std::borrow::Cow;
use web_sys::{console, EventSourceInit};

#[cfg(feature = "ssr")]
mod axum;
#[cfg(feature = "ssr")]
pub use axum::*;

pub mod signals;

/// Creates a signal which is controlled by the server.
///
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
pub fn sse_sync_maindata(url: &str) -> ArcReadSignal<SyncState> {
    let (data, set) = arc_signal(SyncState::default());

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
    let mut opts = EventSourceInit::new();
    opts.with_credentials(true);
    let es = EventSource::new_with_event_source_init_dict(url, &opts).unwrap();

    let internal_data = data.clone();
    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let ws_string = event
            .data()
            .dyn_into::<JsString>()
            .unwrap()
            .as_string()
            .unwrap();
        let result = serde_json::from_str::<MainData>(&ws_string);
        tracing::info!(event = ?event);
        match result {
            Ok(MainData::Full(full_data)) => set.set(SyncState::from(full_data)),
            Ok(MainData::Partial(partial_data)) => {
                internal_data.with_untracked(|data| {
                    if let Some(torrents) = partial_data.torrents {
                        for (hash, partial) in torrents {
                            data.torrents
                                .get(&hash)
                                .map(|torrent| torrent.apply_partial(partial));
                        }
                    }
                    if let Some(server_state) = partial_data.server_state {
                        data.server_state.apply_partial(server_state);
                    }
                });
            }
            Err(err) => {
                tracing::error!(error = ?err);
            }
        }
    }) as Box<dyn FnMut(_)>);
    let function: &Function = callback.as_ref().unchecked_ref();
    es.set_onmessage(Some(function));

    let err_callback = Closure::wrap(Box::new(move |event: JsValue| {
        let vals = Array::new();
        vals.push(&event);
        console::log(&vals);
    }) as Box<dyn FnMut(_)>);
    let err_func: &Function = err_callback.as_ref().unchecked_ref();

    es.set_onerror(Some(err_func));
    callback.forget();

     } else {
            tracing::error!(
                r#"server signal was used without a SSE being provided.

    Ensure you call `leptos_sse::provide_sse("http://localhost:3000/sse")` at the highest level in your app."#
            );
        }
        }

    data
}
