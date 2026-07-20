use crate::components::common::ClassBuilder;
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
    let class_string = ClassBuilder::new("el-select-group")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

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
