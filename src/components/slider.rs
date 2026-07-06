use dioxus::prelude::*;

/// Slider props - 滑块
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Current value (controlled)
    #[props(default = 0)]
    pub model_value: u32,

    /// Minimum value
    #[props(default = 0)]
    pub min: u32,

    /// Maximum value
    #[props(default = 100)]
    pub max: u32,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Step size
    #[props(default = 1)]
    pub step: u32,

    /// Whether to show input
    #[props(default = false)]
    pub show_input: bool,

    /// Whether to show stops
    #[props(default = false)]
    pub show_stops: bool,

    /// Whether to show tooltip
    #[props(default = true)]
    pub show_tooltip: bool,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<u32>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Slider component - draggable slider
///
/// Mirrors Element Plus `el-slider`.
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut class_names = vec!["el-slider".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let range = props.max - props.min;
    let pct = if range > 0 { ((props.model_value - props.min) as f64 / range as f64) * 100.0 } else { 0.0 };

    let on_change = props.on_change;
    let disabled = props.disabled;
    let min = props.min;
    let max = props.max;
    let step = props.step;
    let current = props.model_value;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            div {
                class: "el-slider__runway",

                div {
                    class: "el-slider__bar",
                    style: "width: {pct}%;",
                }

                div {
                    class: "el-slider__button-wrapper",
                    style: "left: {pct}%;",

                    div {
                        class: "el-slider__button",
                    }
                }
            }

            if props.show_input {
                input {
                    class: "el-slider__input",
                    r#type: "range",
                    min: "{min}",
                    max: "{max}",
                    step: "{step}",
                    value: "{current}",
                    disabled: disabled,
                    oninput: move |e: Event<FormData>| {
                        if !disabled {
                            if let Ok(v) = e.value().parse::<u32>() {
                                if let Some(handler) = on_change.as_ref() {
                                    handler.call(v);
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
