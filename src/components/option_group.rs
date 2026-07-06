use dioxus::prelude::*;

/// OptionGroup props
#[derive(Props, Clone, PartialEq)]
pub struct OptionGroupProps {
    /// Group label
    pub label: String,

    /// Group content (options)
    #[props(default)]
    pub children: Element,

    /// Whether the group is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// OptionGroup component for grouping options in a Select
#[component]
pub fn OptionGroup(props: OptionGroupProps) -> Element {
    let mut class_names = vec!["el-select-group".to_string()];

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "el-select-group__wrap",
            div {
                class: "el-select-group__title",
                "{props.label}"
            }
            div {
                class: "{class_string}",
                {props.children}
            }
        }
    }
}
