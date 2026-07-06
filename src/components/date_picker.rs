use dioxus::prelude::*;

/// DatePicker props
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = "Select Date".to_string())]
    pub placeholder: String,

    #[props(default = "date".to_string())]
    pub picker_type: String,

    #[props(default = "YYYY-MM-DD".to_string())]
    pub format: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default = false)]
    pub readonly: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// DatePicker component for date selection
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let mut class_names = vec!["el-date-editor".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            div {
                class: "el-input el-input--default el-range-editor",
                input {
                    class: "el-input__inner",
                    r#type: "text",
                    placeholder: "{props.placeholder}",
                    value: props.model_value.clone().unwrap_or_default(),
                    disabled: props.disabled,
                    readonly: props.readonly,
                    onchange: move |e| {
                        if let Some(handler) = props.on_change {
                            handler.call(e.value());
                        }
                    },
                }
                span {
                    class: "el-input__prefix",
                    i { class: "el-icon-date" }
                }
            }
        }
    }
}
