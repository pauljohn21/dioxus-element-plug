use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// Collapse props
#[derive(Props, Clone, PartialEq)]
pub struct CollapseProps {
    #[props(default)]
    pub children: Element,

    /// Currently expanded item values
    #[props(default)]
    pub active_names: Vec<String>,

    /// Whether to allow multiple expanded items
    #[props(default = false)]
    pub accordion: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Collapse component for collapsible panels
#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let class_string = ClassBuilder::new("el-collapse")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "tablist",
            {props.children}
        }
    }
}

/// CollapseItem props
#[derive(Props, Clone, PartialEq)]
pub struct CollapseItemProps {
    #[props(default)]
    pub children: Element,

    /// Unique name for this item
    pub name: String,

    /// Title text
    pub title: String,

    /// Whether this item is disabled
    #[props(default = false)]
    pub disabled: bool,

    #[props(default)]
    pub on_click: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// CollapseItem component for individual collapsible sections
#[component]
pub fn CollapseItem(props: CollapseItemProps) -> Element {
    let name_clone = props.name.clone();
    let style_string = style_str(&props.style);
    let on_click = props.on_click;
    rsx! {
        div {
            class: "el-collapse-item",
            style: "{style_string}",
            div {
                class: "el-collapse-item__header",
                role: "tab",
                onclick: move |_| {
                    if !props.disabled {
                        fire_event(&on_click, name_clone.clone());
                    }
                },
                "{props.title}"
                i { class: "el-collapse-item__arrow el-icon-arrow-right" }
            }
            div {
                class: "el-collapse-item__wrap",
                div {
                    class: "el-collapse-item__content",
                    {props.children}
                }
            }
        }
    }
}
