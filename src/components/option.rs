use crate::components::common::ClassBuilder;
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
    let class_string = ClassBuilder::new("el-select-dropdown__item")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    rsx! {
        div {
            class: "{class_string}",
            "{props.label}"
        }
    }
}
