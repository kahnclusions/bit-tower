use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Navbar(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <nav class=tw_merge!("fixed top-0 left-0 right-0 h-9 bg-background_highlight flex flex-row justify-between items-center", class)>
        {children()}
        </nav>
    }
}

#[component]
pub fn NavbarBrand(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=tw_merge!(" p-2 ", class)>
        {children()}
        </div>
    }
}
