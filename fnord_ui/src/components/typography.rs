use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn H1(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H1>,
    children: Children,
) -> impl IntoView {
    view! {
        <h1
            class=tw_merge!("font-bold scroll-m-20 mt-5 text-4xl tracking-tight lg:text-5xl", class)
            node_ref=node_ref
        >
            {children()}
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H2>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2
            class=tw_merge!(
                "font-bold mt-5 scroll-m-20 border-b pb-2 text-3xl tracking-tight transition-colors first:mt-0",
                class
            )

            node_ref=node_ref
        >
            {children()}
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H3>,
    children: Children,
) -> impl IntoView {
    view! {
        <h3
            class=tw_merge!("mt-8 scroll-m-20 text-2xl font-semibold tracking-tight", class)
            node_ref=node_ref
        >
            {children()}
        </h3>
    }
}

#[component]
pub fn Text(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::P>,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=tw_merge!("leading-7", class) node_ref=node_ref>
            {children()}
        </p>
    }
}

#[component]
pub fn TextSpan(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=tw_merge!("leading-7", class) node_ref=node_ref>
            {children()}
        </span>
    }
}
