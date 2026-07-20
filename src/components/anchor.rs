use dioxus::prelude::*;

use crate::components::common::{fire_event, style_str, ClassBuilder};

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
    let direction_class = format!("el-anchor--{}", props.direction);
    let class_string = ClassBuilder::new("el-anchor")
        .add_class(&direction_class)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
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
    let class_string = ClassBuilder::new("el-anchor__link")
        .add_opt(props.class.as_ref())
        .build();
    let href_clone = props.href.clone();
    let on_click = props.on_click;
    let disabled = props.disabled;

    rsx! {
        div {
            class: "{class_string}",
            a {
                class: "el-anchor__link-title",
                href: "{props.href}",
                onclick: move |e| {
                    e.prevent_default();
                    if !disabled {
                        fire_event(&on_click, href_clone.clone());
                    }
                },
                "{props.title}"
            }
            {props.children}
        }
    }
}
