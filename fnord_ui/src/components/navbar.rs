use leptos::prelude::*;

#[component]
pub fn Navbar(children: Children) -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 h-9 bg-background-highlight">
        {children()}
        </nav>
    }
}

#[component]
pub fn NavbarBrand(children: Children) -> impl IntoView {
    view! {
        <div class="fixed top-0 left-0 right-0 h-9 bg-background-highlight">
        {children()}
        </div>
    }
}
