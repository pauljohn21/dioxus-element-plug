use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str};

/// TimeSelect props
#[derive(Props, Clone, PartialEq)]
pub struct TimeSelectProps {
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = "Select Time".to_string())]
    pub placeholder: String,

    #[props(default = "00:00".to_string())]
    pub start: String,

    #[props(default = "23:59".to_string())]
    pub end: String,

    #[props(default = "00:30".to_string())]
    pub step: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// TimeSelect component for fixed-time selection
#[component]
pub fn TimeSelect(props: TimeSelectProps) -> Element {
    let class_string = ClassBuilder::new("el-time-select")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-input",
                input {
                    class: "el-input__inner",
                    r#type: "text",
                    placeholder: "{props.placeholder}",
                    value: props.model_value.clone().unwrap_or_default(),
                    disabled: props.disabled,
                    readonly: true,
                }
                span {
                    class: "el-input__prefix",
                    i { class: "el-icon-time" }
                }
            }
        }
    }
}
