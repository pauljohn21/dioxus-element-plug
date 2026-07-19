use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Rate size variants
#[derive(Clone, PartialEq)]
pub enum RateSize {
    Large,
    Default,
    Small,
}

impl RateSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            RateSize::Large => "el-rate--large",
            RateSize::Default => "",
            RateSize::Small => "el-rate--small",
        }
    }
}

/// Rate props
#[derive(Props, Clone, PartialEq)]
pub struct RateProps {
    /// Current rating value
    #[props(default = 0.0)]
    pub model_value: f64,

    /// Maximum rating value (number of stars)
    #[props(default = 5)]
    pub max: u32,

    /// Whether the rate is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to allow half stars
    #[props(default = false)]
    pub allow_half: bool,

    /// Whether to allow clearing (click again to clear)
    #[props(default = false)]
    pub clearable: bool,

    /// Whether to show text
    #[props(default = false)]
    pub show_text: bool,

    /// Whether to show score
    #[props(default = false)]
    pub show_score: bool,

    /// Text array for each rating level
    #[props(default)]
    pub texts: Vec<String>,

    /// Color for active stars
    #[props(default = "#F7BA2A".to_string())]
    pub active_color: String,

    /// Color for inactive stars
    #[props(default = "#C6D1DE".to_string())]
    pub inactive_color: String,

    /// Rate size
    #[props(default = RateSize::Default)]
    pub size: RateSize,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<f64>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Rate component for star ratings
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Rate {
///         model_value: 3.5,
///         allow_half: true,
///         max: 5,
///         on_change: move |v| println!("Rating: {}", v),
///     }
/// }
/// ```
#[component]
pub fn Rate(props: RateProps) -> Element {
    let class_string = ClassBuilder::new("el-rate")
        .add_class(props.size.as_class())
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;

    let current_value = props.model_value;
    let text_index = if current_value > 0.0 {
        (current_value.ceil() as usize).saturating_sub(1).min(props.texts.len().saturating_sub(1))
    } else {
        0
    };

    let display_text = if props.show_text && !props.texts.is_empty() {
        props.texts.get(text_index).cloned().unwrap_or_default()
    } else if props.show_score {
        format!("{}", current_value)
    } else {
        String::new()
    };

    let stars: Vec<(f64, String, &'static str)> = (1..=props.max)
        .map(|i| {
            let star_value = i as f64;
            let is_active = star_value <= current_value;
            let is_half = props.allow_half && star_value - 0.5 == current_value;
            let color = if is_active { props.active_color.clone() } else { props.inactive_color.clone() };
            let star_char = if is_active { "★" } else if is_half { "⯨" } else { "☆" };
            (star_value, color, star_char)
        })
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "slider",
            aria_valuenow: "{current_value}",
            aria_valuemax: "{props.max}",
            for (star_value, color, star_char) in stars.into_iter() {
                span {
                    class: "el-rate__item",
                    style: "cursor: pointer;",
                    onclick: move |_| {
                        if !props.disabled {
                            if props.clearable && star_value == current_value {
                                fire_event(&on_change, 0.0);
                            } else {
                                fire_event(&on_change, star_value);
                            }
                        }
                    },
                    i {
                        class: "el-rate__icon",
                        style: "color: {color};",
                        "{star_char}"
                    }
                }
            }
            if props.show_text || props.show_score {
                span {
                    class: "el-rate__text",
                    "{display_text}"
                }
            }
        }
    }
}
