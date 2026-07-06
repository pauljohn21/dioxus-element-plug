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
    let mut class_names = vec!["el-infinite-scroll".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
            div {
                class: "el-infinite-scroll__loading",
                if props.has_more {
                    span {
                        class: "el-infinite-scroll__text",
                        onclick: move |_| {
                            if let Some(handler) = props.on_load {
                                handler.call(());
                            }
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
