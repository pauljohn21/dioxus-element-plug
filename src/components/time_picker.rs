use dioxus::prelude::*;

/// TimePicker props
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = "Select Time".to_string())]
    pub placeholder: String,

    #[props(default = "HH:mm:ss".to_string())]
    pub format: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default = false)]
    pub is_range: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// TimePicker component for time selection
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let mut class_names = vec!["el-date-editor".to_string(), "el-time-editor".to_string()];
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
