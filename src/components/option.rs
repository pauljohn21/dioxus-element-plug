use dioxus::prelude::*;

/// Option component for use inside Select
#[derive(Props, Clone, PartialEq)]
pub struct OptionProps {
    /// Option value
    pub value: String,

    /// Option label
    pub label: String,

    /// Whether the option is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Option component - represents a single option in a Select
#[component]
pub fn OptionComponent(props: OptionProps) -> Element {
    let mut class_names = vec!["el-select-dropdown__item".to_string()];

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            "{props.label}"
        }
    }
}
