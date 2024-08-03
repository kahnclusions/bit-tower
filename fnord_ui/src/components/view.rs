use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn View(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("flex flex-col gap-6", class) node_ref=node_ref>
            {children()}
        </div>
    }
}
