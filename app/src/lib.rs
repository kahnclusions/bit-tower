pub mod auth;
mod routes;

// mod use_websocket;

mod components;
mod signals;

use crate::error_template::{AppError, ErrorTemplate};
use auth::{has_auth, Login};
use components::{status_bar::StatusBar, torrents::TorrentList};
use icondata as i;
use leptos::text_prop::TextProp;
use leptos::{either::Either, prelude::*};
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

use fnord_ui::components::{Button, Navbar, NavbarBrand, Text, View};
use serde::{Deserialize, Serialize};
use signals::syncstate::{ServerState, SyncState, Torrent};
use signals::use_sync_maindata::{use_sync_maindata, UseSyncMaindataReturn};
use use_websocket::core::ConnectionReadyState;

pub mod error_template;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="text-gray-950 bg-gray-50 dark:bg-gray-950 dark:text-gray-50 overscroll-none">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let login = ServerAction::<Login>::new();
    let is_auth = Resource::new(move || login.version(), move |_| has_auth());
    let auth = Signal::derive(move || is_auth.get().map(|v| v.unwrap_or(false)).unwrap_or(false));

    // let torrents = Signal::derive(move || {
    //     let v: Vec<_> = data().torrents.into_iter().map(|(_h, v)| v).collect();
    //     v
    // });
    // let server_state = Signal::derive(move || data().server_state);

    view! {
        <Stylesheet id="leptos" href="/pkg/bittower.css"/>

        // sets the document title
        <Title text="bit-tower"/>

        // content for this welcome page
        <Router>
            <Navbar class="z-50 dark:bg-gray-950 border-b border-t-gray-300 dark:border-b-gray-700">
                <NavbarBrand class="font-display text-cyan-600">"bit-tower"</NavbarBrand>
                <ul class="p-2 font-cubic">
                    <Show when=move || auth.get() fallback=|| view! { <p>hello</p> }>
                    <A href="/menu">menu</A>
                    </Show>
                </ul>
            </Navbar>
            <main class="pt-9">
                <FlatRoutes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }.into_view()
                }>
                    <Route
                        path=StaticSegment("")
                        view=move || {
                            let UseSyncMaindataReturn { ready_state, data, .. } = use_sync_maindata(
                                "ws://10.0.33.171:3010/ws",
                            );
                            view! { <HomePage is_auth=auth action=login data=data ready_state=ready_state /> }
                        }
                    />

                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(
    is_auth: Signal<bool>,
    action: ServerAction<Login>,
    data: ReadSignal<SyncState>,
    ready_state: Signal<ConnectionReadyState>,
) -> impl IntoView {
    let res = move || {
        if is_auth() {
            Either::Left(view! {
                <div class="font-iosevka">
                    <Dashboard data=data  ready_state=ready_state />
                </div>
            })
        } else {
            Either::Right(view! {
                <ActionForm action=action>
                    <h1>"Log In"</h1>
                    <label>
                        "User ID:"
                        <input
                            type="text"
                            placeholder="User ID"
                            maxlength="32"
                            name="username"
                            class="auth-input"
                        />
                    </label>
                    <label>
                        "Password:"
                        <input
                            type="password"
                            placeholder="Password"
                            name="password"
                            class="auth-input"
                        />
                    </label>
                    <button type="submit" class="button">
                        "Log In"
                    </button>
                </ActionForm>
            })
        }
    };

    view! { <div>{res()}</div> }
}

#[component]
fn Dashboard(
    data: ReadSignal<SyncState>,

    ready_state: Signal<ConnectionReadyState>,
) -> impl IntoView {
    let torrents = Signal::derive(move || {
        let v: Vec<_> = data().torrents.into_iter().map(|(_h, v)| v).collect();
        v
    });
    view! {
        <View>
            <TorrentList torrents=torrents/>
            <MobileNavBar />
            {move || view! { <StatusBar server_state=data().server_state ready_state=ready_state /> }}
        </View>
    }
}

#[component]
fn MobileNavBar() -> impl IntoView {
    view! {
        <View class="flex-row gap-0">
            <Button><Icon icon=i::TbMagnet class=TextProp::from("w-4 w-4 text-grey-300") /></Button>
            <Button><Icon icon=i::TbFileUpload class=TextProp::from("w-4 w-4 text-grey-300") /></Button>
        </View>
    }
}
