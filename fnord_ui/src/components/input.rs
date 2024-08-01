use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

pub const INPUT_CLASS: &'static str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Input(
    #[prop(optional)] node_ref: NodeRef<html::Input>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] minlength: String,
    #[prop(optional, into)] maxlength: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] value: String,
    #[prop(default = false)] required: bool,
    #[prop(default = "text".to_string(), into)] html_type: String,
) -> impl IntoView {
    let common = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

    view! {
        <input
            type=html_type
            name=name
            placeholder=placeholder
            node_ref=node_ref
            class=tw_merge!(common, class)
            minlength=minlength
            maxlength=maxlength
            required=required
            value=value
        />
    }
}
