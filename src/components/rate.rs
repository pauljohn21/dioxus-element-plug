use dioxus::prelude::*;

/// Rate props - 评分
#[derive(Props, Clone, PartialEq)]
pub struct RateProps {
    /// Current value (controlled)
    #[props(default = 0)]
    pub model_value: u32,

    /// Maximum rating value
    #[props(default = 5)]
    pub max: u32,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to allow half star
    #[props(default = false)]
    pub allow_half: bool,

    /// Whether to show score text
    #[props(default = false)]
    pub show_score: bool,

    /// Score text color
    #[props(default = "#F7BA2A".to_string())]
    pub active_color: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<u32>>,
}

/// Rate component - star rating
///
/// Mirrors Element Plus `el-rate`.
#[component]
pub fn Rate(props: RateProps) -> Element {
    let mut class_names = vec!["el-rate".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_change = props.on_change;
    let current_value = props.model_value;
    let disabled = props.disabled;

    // Pre-build star data
    let stars: Vec<(u32, bool)> = (1..=props.max)
        .map(|i| (i, i <= current_value))
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            for (value, active) in stars {
                span {
                    class: if active { "el-rate__icon is-active" } else { "el-rate__icon" },
                    style: if active { format!("color: {};", props.active_color) } else { String::new() },
                    onclick: move |_| {
                        if !disabled {
                            if let Some(handler) = on_change.as_ref() {
                                handler.call(value);
                            }
                        }
                    },
                    "★"
                }
            }

            if props.show_score {
                span {
                    class: "el-rate__text",
                    style: "color: {props.active_color};",
                    "{current_value}"
                }
            }
        }
    }
}
