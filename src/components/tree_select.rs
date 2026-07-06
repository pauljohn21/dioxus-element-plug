use dioxus::prelude::*;

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
    let mut class_names = vec!["el-tree-select".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
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
