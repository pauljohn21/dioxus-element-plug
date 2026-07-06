use dioxus::prelude::*;
pub use super::cascader::{CascaderOption, CascaderProps};

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
    let mut class_names = vec!["el-cascader-panel".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
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
