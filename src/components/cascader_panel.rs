use dioxus::prelude::*;
pub use super::cascader::{CascaderOption, CascaderProps};

use crate::components::common::{ClassBuilder, style_str};

/// CascaderPanel props
#[derive(Props, Clone, PartialEq)]
pub struct CascaderPanelProps {
    #[props(default)]
    pub options: Vec<CascaderOption>,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// CascaderPanel component - core panel for cascader
#[component]
pub fn CascaderPanel(props: CascaderPanelProps) -> Element {
    let class_string = ClassBuilder::new("el-cascader-panel")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-cascader-menu",
                for opt in props.options.iter() {
                    div {
                        class: "el-cascader-menu__item",
                        "{opt.label}"
                    }
                }
            }
        }
    }
}
