use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// InfiniteScroll props
#[derive(Props, Clone, PartialEq)]
pub struct InfiniteScrollProps {
    #[props(default)]
    pub children: Element,

    /// Whether there are more items to load
    #[props(default = false)]
    pub has_more: bool,

    /// Loading text
    #[props(default = "Loading...".to_string())]
    pub loading_text: String,

    /// No more data text
    #[props(default = "No more".to_string())]
    pub no_more_text: String,

    /// Distance threshold to trigger load (in px)
    #[props(default = 0)]
    pub distance: u32,

    /// Load more event handler
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// InfiniteScroll component for lazy loading content
#[component]
pub fn InfiniteScroll(props: InfiniteScrollProps) -> Element {
    let class_string = ClassBuilder::new("el-infinite-scroll")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_load = props.on_load;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
            div {
                class: "el-infinite-scroll__loading",
                if props.has_more {
                    span {
                        class: "el-infinite-scroll__text",
                        onclick: move |_| {
                            fire_event(&on_load, ());
                        },
                        "{props.loading_text}"
                    }
                } else {
                    span {
                        class: "el-infinite-scroll__no-more",
                        "{props.no_more_text}"
                    }
                }
            }
        }
    }
}
