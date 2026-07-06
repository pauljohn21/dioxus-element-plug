use dioxus::prelude::*;

/// Switch size
#[derive(Clone, PartialEq)]
pub enum SwitchSize {
    Large,
    Default,
    Small,
}

impl SwitchSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            SwitchSize::Large => "el-switch--large",
            SwitchSize::Default => "",
            SwitchSize::Small => "el-switch--small",
        }
    }
}

/// Switch props - 开关
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether active (controlled)
    #[props(default = false)]
    pub model_value: bool,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether loading
    #[props(default = false)]
    pub loading: bool,

    /// Switch size
    #[props(default = SwitchSize::Default)]
    pub size: SwitchSize,

    /// Custom width
    #[props(default)]
    pub width: Option<String>,

    /// Inline prompt text
    #[props(default = false)]
    pub inline_prompt: bool,

    /// Active text
    #[props(default)]
    pub active_text: Option<String>,

    /// Inactive text
    #[props(default)]
    pub inactive_text: Option<String>,

    /// Active color
    #[props(default = "#409EFF".to_string())]
    pub active_color: String,

    /// Inactive color
    #[props(default = "#C0CCDA".to_string())]
    pub inactive_color: String,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Switch component
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let mut class_names = vec!["el-switch".to_string()];
    class_names.push(props.size.as_class().to_string());

    if props.model_value {
        class_names.push("is-checked".to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if props.loading {
        class_names.push("is-loading".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let core_color = if props.model_value { &props.active_color } else { &props.inactive_color };
    let core_style = format!("background-color: {};", core_color);

    let on_change = props.on_change;
    let checked = props.model_value;
    let disabled = props.disabled;

    rsx! {
        div {
            class: "{class_string}",
            style: "{props.style.as_ref().map(|s| s.as_str()).unwrap_or_default()}",

            if !props.inline_prompt {
                if let Some(ref text) = props.inactive_text {
                    span {
                        class: "el-switch__label el-switch__label--left",
                        style: if !checked { format!("color: {};", props.inactive_color) } else { String::new() },
                        "{text}"
                    }
                }
            }

            span {
                class: "el-switch__core",
                style: "{core_style}",
                onclick: move |_| {
                    if !disabled {
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(!checked);
                        }
                    }
                },

                if props.inline_prompt {
                    span {
                        class: "el-switch__inner",
                        if checked {
                            if let Some(ref text) = props.active_text {
                                "{text}"
                            }
                        } else {
                            if let Some(ref text) = props.inactive_text {
                                "{text}"
                            }
                        }
                    }
                }

                span {
                    class: "el-switch__action",
                }
            }

            if !props.inline_prompt {
                if let Some(ref text) = props.active_text {
                    span {
                        class: "el-switch__label el-switch__label--right",
                        style: if checked { format!("color: {};", props.active_color) } else { String::new() },
                        "{text}"
                    }
                }
            }
        }
    }
}
