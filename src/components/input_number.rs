use dioxus::prelude::*;

/// InputNumber props - 数字输入框
#[derive(Props, Clone, PartialEq)]
pub struct InputNumberProps {
    /// Current value (controlled)
    #[props(default = 0)]
    pub model_value: i64,

    /// Minimum value
    #[props(default = i64::MIN)]
    pub min: i64,

    /// Maximum value
    #[props(default = i64::MAX)]
    pub max: i64,

    /// Step size
    #[props(default = 1)]
    pub step: i64,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Number of precision digits
    #[props(default)]
    pub precision: Option<u32>,

    /// Size
    #[props(default)]
    pub size: Option<InputNumberSize>,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<i64>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// InputNumber size
#[derive(Clone, PartialEq)]
pub enum InputNumberSize {
    Large,
    Default,
    Small,
}

impl InputNumberSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            InputNumberSize::Large => "el-input-number--large",
            InputNumberSize::Default => "",
            InputNumberSize::Small => "el-input-number--small",
        }
    }
}

/// InputNumber component - numeric input with controls
///
/// Mirrors Element Plus `el-input-number`.
#[component]
pub fn InputNumber(props: InputNumberProps) -> Element {
    let mut class_names = vec!["el-input-number".to_string()];

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref s) = props.size {
        class_names.push(s.as_class().to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_change = props.on_change;
    let disabled = props.disabled;
    let min = props.min;
    let max = props.max;
    let step = props.step;
    let current = props.model_value;
    let can_decrease = !disabled && current > min;
    let can_increase = !disabled && current < max;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            button {
                class: if can_decrease { "el-input-number__decrease" } else { "el-input-number__decrease is-disabled" },
                disabled: !can_decrease,
                onclick: move |_| {
                    if can_decrease {
                        let new_val = (current - step).max(min);
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(new_val);
                        }
                    }
                },
                "−"
            }

            input {
                class: "el-input-number__inner",
                r#type: "number",
                value: "{current}",
                disabled: disabled,
                onchange: move |e: Event<FormData>| {
                    if let Ok(v) = e.value().parse::<i64>() {
                        let clamped = v.clamp(min, max);
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(clamped);
                        }
                    }
                },
            }

            button {
                class: if can_increase { "el-input-number__increase" } else { "el-input-number__increase is-disabled" },
                disabled: !can_increase,
                onclick: move |_| {
                    if can_increase {
                        let new_val = (current + step).min(max);
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(new_val);
                        }
                    }
                },
                "+"
            }
        }
    }
}
