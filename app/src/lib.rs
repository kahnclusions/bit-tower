pub mod auth;
mod routes;

// mod use_websocket;

mod components;
mod signals;

use crate::error_template::{AppError, ErrorTemplate};
use auth::{has_auth, Login};
use components::{status_bar::StatusBar, torrents::TorrentList};
use leptos::{either::Either, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

use fnord_ui::components::{Navbar, NavbarBrand, Text, View};
use serde::{Deserialize, Serialize};
use signals::syncstate::{ServerState, SyncState, Torrent};
use signals::use_sync_maindata::{use_sync_maindata, UseSyncMaindataReturn};

pub mod error_template;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags />
            </head>
            <body class="bg-background text-foreground ">
                <App />
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
            <Navbar class="z-50">
                <NavbarBrand class="font-display text-green1">"bit-tower"</NavbarBrand>
                <ul class="p-2 font-cubic">
                    <A href="/login">login</A>
                </ul>
            </Navbar>
            <main class="pt-9 bg-background">
                <FlatRoutes fallback=|| {

            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
                    <Route path=StaticSegment("") view=move || {
    let UseSyncMaindataReturn {
        connected, data, ..
    } = use_sync_maindata("wss://local.ck.dev/ws");

    let status = move || {
        if connected.get() {
            "Connected"
        } else {
            "Disconnected"
        }
    };

    Effect::new(move |_| {
        tracing::info!("Status = {}", status());
    });

            view! {


                        <HomePage is_auth=auth action=login data=data />

                    }} />
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
) -> impl IntoView {
    let res = move || {
        if is_auth() {
            Either::Left(view! {
                <div><Dashboard data=data /></div>
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
                        <input type="password" placeholder="Password" name="password" class="auth-input"/>
                    </label>
                    <button type="submit" class="button">
                        "Log In"
                    </button>
                </ActionForm>
            })
        }
    };

    view! {
       <div>{res()}</div>
    }
}

#[component]
fn Dashboard(data: ReadSignal<SyncState>) -> impl IntoView {
    let torrents = Signal::derive(move || {
        let v: Vec<_> = data().torrents.into_iter().map(|(_h, v)| v).collect();
        v
    });
    view! {
        <View>
        <TorrentList torrents=torrents />
        {move || view! {<StatusBar server_state=data().server_state />}}
        </View>
    }
}
