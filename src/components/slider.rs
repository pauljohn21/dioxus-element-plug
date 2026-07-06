use dioxus::prelude::*;

/// Slider props
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Current value
    #[props(default = 0.0)]
    pub model_value: f64,

    /// Minimum value
    #[props(default = 0.0)]
    pub min: f64,

    /// Maximum value
    #[props(default = 100.0)]
    pub max: f64,

    /// Step size
    #[props(default = 1.0)]
    pub step: f64,

    /// Whether the slider is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to show input
    #[props(default = false)]
    pub show_input: bool,

    /// Whether to show stops
    #[props(default = false)]
    pub show_stops: bool,

    /// Whether to show tooltip
    #[props(default = true)]
    pub show_tooltip: bool,

    /// Slider orientation
    #[props(default = "horizontal".to_string())]
    pub direction: String,

    /// Whether to allow range selection
    #[props(default = false)]
    pub range: bool,

    /// Format tooltip text
    #[props(default)]
    pub format_tooltip: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<f64>>,

    /// Input event handler
    #[props(default)]
    pub on_input: Option<EventHandler<f64>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Slider component for selecting a value from a range
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Slider {
///         model_value: 50.0,
///         min: 0.0,
///         max: 100.0,
///         on_change: move |v| println!("Value: {}", v),
///     }
/// }
/// ```
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut class_names = vec!["el-slider".to_string()];

    class_names.push(format!("is-{}", props.direction));

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if props.show_input {
        class_names.push("el-slider--with-input".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();

    let percentage = if props.max > props.min {
        ((props.model_value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let bar_style = format!("width: {}%;", percentage);
    let button_style = format!("left: {}%;", percentage);

    let stops = if props.show_stops && props.step > 0.0 {
        let mut stops_vec = Vec::new();
        let mut val = props.min;
        while val < props.max {
            let stop_pct = ((val - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0);
            stops_vec.push(stop_pct);
            val += props.step;
        }
        stops_vec
    } else {
        vec![]
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "slider",
            aria_valuemin: "{props.min}",
            aria_valuemax: "{props.max}",
            aria_valuenow: "{props.model_value}",
            aria_disabled: "{props.disabled}",
            div {
                class: "el-slider__runway",
                div {
                    class: "el-slider__bar",
                    style: "{bar_style}",
                }
                for stop_pct in stops.iter() {
                    div {
                        class: "el-slider__stop",
                        style: "left: {stop_pct}%;",
                    }
                }
                div {
                    class: "el-slider__button-wrapper",
                    style: "{button_style}",
                    div {
                        class: "el-slider__button",
                    }
                    if props.show_tooltip {
                        div {
                            class: "el-slider__tooltip",
                            "{props.model_value}"
                        }
                    }
                }
            }
            if props.show_input {
                div {
                    class: "el-slider__input",
                    input {
                        r#type: "number",
                        value: "{props.model_value}",
                        min: "{props.min}",
                        max: "{props.max}",
                        step: "{props.step}",
                        disabled: props.disabled,
                        onchange: move |e| {
                            if let Some(handler) = props.on_change {
                                if let Ok(val) = e.value().parse::<f64>() {
                                    handler.call(val);
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
