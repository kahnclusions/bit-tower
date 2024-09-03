use super::hashed_stylesheet::HashedStylesheet;
use super::hydration::HydrationScripts;
use bittower::app::App;
use leptos::prelude::*;
use leptos_meta::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <HashedStylesheet id="leptos" options />
                <MetaTags/>
            </head>
            <body class="text-gray-950 bg-gray-50 dark:bg-gray-950 dark:text-gray-50 overscroll-none">
                <App/>
            </body>
        </html>
    }
}
