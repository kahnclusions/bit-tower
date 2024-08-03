pub mod auth;
mod routes;

use crate::error_template::{AppError, ErrorTemplate};

use auth::{has_auth, Login};
use leptos::{either::Either, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

use fnord_ui::components::{Navbar, NavbarBrand};
use serde::{Deserialize, Serialize};

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

    view! {
        <Stylesheet id="leptos" href="/pkg/bittower.css"/>

        // sets the document title
        <Title text="bit-tower"/>

        // content for this welcome page
        <Router>
            <Navbar>
                <NavbarBrand class="font-display">"bit-tower"</NavbarBrand>
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
                    <Route path=StaticSegment("") view=move || view! { <HomePage is_auth=auth action=login /> } />
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(is_auth: Signal<bool>, action: ServerAction<Login>) -> impl IntoView {
    let res = move || {
        if is_auth() {
            Either::Left(view! {
                <div><Dashboard /></div>
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
fn Dashboard() -> impl IntoView {
    // Provide websocket connection
    // leptos_sse::provide_sse("http://localhost:3000/sse").unwrap();

    use qbittorrent_rs_sse::sse_sync_maindata;
    // Create sse signal
    let data = sse_sync_maindata("/sse");

    view! {
        <div>Count: {move || { view! { <div>
            <div>"DL: "{data().server_state.dl_info_speed.to_string()}</div>
            <div>"UP: "{data().server_state.up_info_speed.to_string()}</div>
            <div>"Status: "{data().server_state.connection_status.to_string()}</div>
            </div> }}}</div>
    }
}
