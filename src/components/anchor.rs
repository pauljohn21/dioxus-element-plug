use dioxus::prelude::*;

/// Anchor props
#[derive(Props, Clone, PartialEq)]
pub struct AnchorProps {
    #[props(default)]
    pub children: Element,

    /// Current active anchor
    #[props(default)]
    pub current: Option<String>,

    /// Anchor direction
    #[props(default = "vertical".to_string())]
    pub direction: String,

    /// Container selector
    #[props(default)]
    pub container: Option<String>,

    /// Offset for triggering active state
    #[props(default = 100)]
    pub offset: u32,

    /// Scroll offset
    #[props(default = 0)]
    pub scroll_offset: u32,

    /// Bound element selector
    #[props(default)]
    pub bound: Option<u32>,

    #[props(default)]
    pub on_click: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Anchor component for anchor navigation
#[component]
pub fn Anchor(props: AnchorProps) -> Element {
    let mut class_names = vec!["el-anchor".to_string()];
    class_names.push(format!("el-anchor--{}", props.direction));
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            div {
                class: "el-anchor__wrapper",
                {props.children}
            }
        }
    }
}

/// AnchorLink props
#[derive(Props, Clone, PartialEq)]
pub struct AnchorLinkProps {
    /// Link text
    pub title: String,

    /// Anchor href
    pub href: String,

    #[props(default)]
    pub children: Option<Element>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default)]
    pub on_click: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,
}

/// AnchorLink component for individual anchor links
#[component]
pub fn AnchorLink(props: AnchorLinkProps) -> Element {
    let href_clone = props.href.clone();
    rsx! {
        div {
            class: "el-anchor__link {props.class.clone().unwrap_or_default()}",
            a {
                class: "el-anchor__link-title",
                href: "{props.href}",
                onclick: move |e| {
                    e.prevent_default();
                    if !props.disabled {
                        if let Some(handler) = props.on_click {
                            handler.call(href_clone.clone());
                        }
                    }
                },
                "{props.title}"
            }
            {props.children}
        }
    }
}
