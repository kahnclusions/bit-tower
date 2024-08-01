mod auth;
mod routes;

use crate::error_template::{AppError, ErrorTemplate};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

use fnord_ui::components::{Navbar, NavbarBrand};

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
            <body class="bg-white text-slate-950 dark:bg-slate-950 dark:text-white">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bittower.css"/>

        // sets the document title
        <Title text="bit-tower"/>

        // content for this welcome page
        <Router>
            <Navbar>
                <NavbarBrand>"bit-tower"</NavbarBrand>
            </Navbar>
            <main class="pt-9">
                <FlatRoutes fallback=|| {

            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
                    <Route path=StaticSegment("") view=HomePage/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class="font-bold">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
