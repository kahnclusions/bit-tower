use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::tw_merge;
use tailwind_fuse::*;

// Your Component Type
#[derive(TwClass)]
// Optional base class
#[tw(class = "flex")]
pub struct BtnVariants {
    pub size: BtnSize,
    pub variant: BtnVariant,
}

// Variant for size
#[derive(TwVariant)]
pub enum BtnSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
    #[tw(class = "h-10 w-10")]
    Icon,
}

// Variant for color
#[derive(TwVariant)]
pub enum BtnVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[component]
pub fn Button(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: BtnSize,
    #[prop(optional)] variant: BtnVariant,
    #[prop(optional, into)] html_type: String,
    #[prop(optional, into)] href: String,
    #[prop(optional, into)] pending: Option<ReadSignal<bool>>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let variants = BtnVariants { size, variant };
    let common = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 max-sm:w-full";
    let children = StoredValue::new(children);
    let pending = move || match pending {
        Some(pending) => pending(),
        None => false,
    };
    let disabled = move || match disabled {
        Some(disabled) => disabled(),
        None => false,
    };

    if !href.is_empty() {
        Either::Left(view! {
            <A attr:id=id href=href attr:class=tw_merge!(common, variants.to_class(), class)>
                {children.with_value(|children| children())}
            </A>
        })
    } else {
        Either::Right(view! {
                <button
                    id=id
                    type=html_type
                    class=tw_merge!(common, variants.to_class(), class)
                    disabled=move || { pending() || disabled() }
                >
                    <Show
                        when=move || !pending()
                        fallback=|| {
                            view! {
                                <div class="loader-small border-white border-b-[#aaa] dark:border-black"></div>
                            }
                        }
                    >
                        {children.with_value(|children| children())}
                    </Show>
                </button>
        })
    }
}
