#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use json_patch::Patch;
use leptos::prelude::{arc_signal, ArcReadSignal, ArcRwSignal, Read, ReadSignal, RwSignal, Update};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[cfg(feature = "ssr")]
mod axum;
#[cfg(feature = "ssr")]
pub use axum::*;

/// A server signal update containing the signal type name and json patch.
///
/// This is whats sent over the SSE, and is used to patch the signal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerSignalUpdate {
    name: Cow<'static, str>,
    patch: Patch,
}

impl ServerSignalUpdate {
    /// Creates a new [`ServerSignalUpdate`] from an old and new instance of `T`.
    pub fn new<T>(
        name: impl Into<Cow<'static, str>>,
        old: &T,
        new: &T,
    ) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        let left = serde_json::to_value(old)?;
        let right = serde_json::to_value(new)?;
        let patch = json_patch::diff(&left, &right);
        Ok(ServerSignalUpdate {
            name: name.into(),
            patch,
        })
    }

    /// Creates a new [`ServerSignalUpdate`] from two json values.
    pub fn new_from_json<T>(name: impl Into<Cow<'static, str>>, old: &Value, new: &Value) -> Self {
        let patch = json_patch::diff(old, new);
        ServerSignalUpdate {
            name: name.into(),
            patch,
        }
    }
}

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
pub fn sse_signal<T>(url: &str) -> ArcReadSignal<T>
where
    T: Clone
        + Default
        + Serialize
        + for<'de> Deserialize<'de>
        + std::marker::Send
        + std::marker::Sync
        + 'static,
{
    let (get, set) = arc_signal(T::default());

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
        use serde_json::Value;
        use wasm_bindgen::{prelude::Closure, JsValue, JsCast};
        use web_sys::{EventSource, MessageEvent};
        use json_patch::Patch;
        use js_sys::{Function, JsString};
        use leptos::prelude::{create_effect, use_context, RwSignal};

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ServerSignalEventSource(pub EventSource);

        let mut delayed_updates: Arc<Mutex<Vec<Patch>>> = Arc::new(Mutex::new(Vec::new()));
        let es = EventSource::new(url).unwrap();

        let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            let ws_string = event
                .data()
                .dyn_into::<JsString>()
                .unwrap()
                .as_string()
                .unwrap();
            if let Ok(update_signal) = serde_json::from_str::<ServerSignalUpdate>(&ws_string) {
                set.update(|doc| {
                    let mut value = serde_json::to_value(doc.clone()).unwrap();
                    let delayed_updates = delayed_updates.clone();
                    let mut delayed_updates = delayed_updates.lock().unwrap();
                    if (*delayed_updates).len() > 0 {
                        for patch in delayed_updates.iter() {
                            json_patch::patch(&mut value, &update_signal.patch).unwrap();
                        }
                        *delayed_updates = vec![];
                    }
                    json_patch::patch(&mut value, &update_signal.patch).unwrap();
                    let new_value: T = serde_json::from_value(value).unwrap();
                    *doc = new_value
                });
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
