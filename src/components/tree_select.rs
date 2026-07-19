use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str};

/// TreeSelect props
#[derive(Props, Clone, PartialEq)]
pub struct TreeSelectProps {
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = "Select".to_string())]
    pub placeholder: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default = false)]
    pub check_strictly: bool,

    #[props(default = "select".to_string())]
    pub select_type: String,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// TreeSelect component for selecting from a tree structure
#[component]
pub fn TreeSelect(props: TreeSelectProps) -> Element {
    let class_string = ClassBuilder::new("el-tree-select")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-select el-select__wrapper",
                onclick: move |_| {
                    // Toggle dropdown
                },
                span {
                    class: "el-select__selected-item",
                    if props.model_value.is_some() {
                        "{props.model_value.as_ref().unwrap()}"
                    } else {
                        "{props.placeholder}"
                    }
                }
                i { class: "el-icon-arrow-down" }
            }
        }
    }
}
