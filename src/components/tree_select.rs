use dioxus::prelude::*;

use crate::components::common::{fire_event, style_str, ClassBuilder};
use crate::components::tree::{Tree, TreeNodeData, TreeProps};

/// TreeSelect props
#[derive(Props, Clone, PartialEq)]
pub struct TreeSelectProps {
    /// Controlled selected value
    #[props(default)]
    pub model_value: Option<String>,

    /// Tree data
    #[props(default)]
    pub tree_data: Vec<TreeNodeData>,

    #[props(default = "Select".to_string())]
    pub placeholder: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default = false)]
    pub check_strictly: bool,

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
    let mut dropdown_open = use_signal(|| false);

    let class_string = ClassBuilder::new("el-tree-select")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let display_text = props
        .model_value
        .clone()
        .unwrap_or_else(|| props.placeholder.clone());
    let has_value = props.model_value.is_some();
    let on_change = props.on_change;
    let disabled = props.disabled;
    let clearable = props.clearable;
    let tree_data = props.tree_data.clone();
    let cursor = if disabled { "not-allowed" } else { "pointer" };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-select el-select__wrapper",
                style: "cursor: {cursor};",
                onclick: move |_| {
                    if !disabled {
                        dropdown_open.set(!dropdown_open());
                    }
                },
                span {
                    class: "el-select__selected-item",
                    style: if !has_value { "color: #a8abb2;" } else { "" },
                    "{display_text}"
                }
                span {
                    class: "el-select__suffix",
                    if clearable && has_value {
                        i {
                            class: "el-input__icon el-icon-circle-close",
                            onclick: move |e: MouseEvent| {
                                e.stop_propagation();
                                fire_event(&on_change, String::new());
                                dropdown_open.set(false);
                            },
                        }
                    } else {
                        i { class: "el-input__icon el-icon-arrow-down" }
                    }
                }
            }
            if dropdown_open() && !disabled {
                div {
                    class: "el-select__popper el-popper",
                    style: "position: absolute; z-index: 2001; background: #fff; border: 1px solid #e4e7ed; border-radius: 4px; box-shadow: 0 0 12px rgba(0,0,0,0.12); padding: 8px; min-width: 200px; max-height: 300px; overflow-y: auto;",
                    Tree {
                        data: tree_data.clone(),
                        on_node_click: move |label: String| {
                            fire_event(&on_change, label);
                            dropdown_open.set(false);
                        },
                    }
                }
            }
        }
    }
}
