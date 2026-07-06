use dioxus::prelude::*;

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
    let mut class_names = vec!["el-time-select".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
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
