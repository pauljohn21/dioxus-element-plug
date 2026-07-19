use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Switch size variants
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

/// Switch props
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is active
    #[props(default = false)]
    pub model_value: bool,

    /// Whether the switch is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the switch is in loading state
    #[props(default = false)]
    pub loading: bool,

    /// Switch size
    #[props(default = SwitchSize::Default)]
    pub size: SwitchSize,

    /// Switch width (e.g., "40px")
    #[props(default)]
    pub width: Option<String>,

    /// Whether to show text inside the dot
    #[props(default = false)]
    pub inline_prompt: bool,

    /// Text displayed when in "on" state
    #[props(default)]
    pub active_text: Option<String>,

    /// Text displayed when in "off" state
    #[props(default)]
    pub inactive_text: Option<String>,

    /// Icon CSS class when in "on" state
    #[props(default)]
    pub active_icon: Option<String>,

    /// Icon CSS class when in "off" state
    #[props(default)]
    pub inactive_icon: Option<String>,

    /// Input name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler (receives new value)
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Switch component for toggling between two states
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Switch {
///         model_value: true,
///         active_text: Some("ON".to_string()),
///         inactive_text: Some("OFF".to_string()),
///         on_change: move |val| println!("Switch: {}", val),
///     }
/// }
/// ```
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let class_string = ClassBuilder::new("el-switch")
        .add_class(props.size.as_class())
        .add_if("is-checked", props.model_value)
        .add_if("is-disabled", props.disabled)
        .add_if("is-loading", props.loading)
        .add_opt(props.class.as_ref())
        .build();

    let mut style_parts = vec![style_str(&props.style)];
    if let Some(ref width) = props.width {
        style_parts.push(format!("width: {};", width));
    }
    let style_string = style_parts.join("");

    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "switch",
            aria_checked: "{props.model_value}",
            onclick: move |_| {
                if !props.disabled && !props.loading {
                    fire_event(&on_change, !props.model_value);
                }
            },
            div {
                class: "el-switch__core",
                if props.inline_prompt {
                    div {
                        class: "el-switch__inner",
                        if props.model_value {
                            if let Some(ref text) = props.active_text {
                                span { class: "el-switch__text", "{text}" }
                            } else if let Some(ref icon) = props.active_icon {
                                i { class: "{icon}" }
                            }
                        } else {
                            if let Some(ref text) = props.inactive_text {
                                span { class: "el-switch__text", "{text}" }
                            } else if let Some(ref icon) = props.inactive_icon {
                                i { class: "{icon}" }
                            }
                        }
                    }
                }
                div {
                    class: "el-switch__action",
                    if props.loading {
                        i { class: "el-icon-loading" }
                    }
                }
            }
            if !props.inline_prompt {
                if props.model_value {
                    if let Some(ref text) = props.active_text {
                        span {
                            class: "el-switch__label el-switch__label--left is-active",
                            "{text}"
                        }
                    }
                } else {
                    if let Some(ref text) = props.inactive_text {
                        span {
                            class: "el-switch__label el-switch__label--right is-active",
                            "{text}"
                        }
                    }
                }
            }
            if let Some(ref name) = props.name {
                input {
                    r#type: "hidden",
                    name: "{name}",
                    value: "{props.model_value}",
                }
            }
        }
    }
}
