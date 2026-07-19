use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// InputNumber props
#[derive(Props, Clone, PartialEq)]
pub struct InputNumberProps {
    #[props(default = 0.0)]
    pub model_value: f64,

    #[props(default = f64::MIN)]
    pub min: f64,

    #[props(default = f64::MAX)]
    pub max: f64,

    #[props(default = 1.0)]
    pub step: f64,

    #[props(default = false)]
    pub step_strictly: bool,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub controls: bool,

    #[props(default = "default".to_string())]
    pub size: String,

    #[props(default)]
    pub precision: Option<u32>,

    #[props(default = "large".to_string())]
    pub controls_position: String,

    #[props(default)]
    pub on_change: Option<EventHandler<f64>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// InputNumber component for numeric input with controls
#[component]
pub fn InputNumber(props: InputNumberProps) -> Element {
    let size_class = format!("el-input-number--{}", props.size);
    let class_string = ClassBuilder::new("el-input-number")
        .add_class(&size_class)
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            if props.controls {
                span {
                    class: "el-input-number__decrease",
                    role: "button",
                    onclick: move |_| {
                        if !props.disabled {
                            let new_val = (props.model_value - props.step).max(props.min);
                            fire_event(&on_change, new_val);
                        }
                    },
                    i { class: "el-icon-minus" }
                }
            }
            div {
                class: "el-input",
                input {
                    class: "el-input__inner",
                    r#type: "number",
                    value: "{props.model_value}",
                    min: "{props.min}",
                    max: "{props.max}",
                    step: "{props.step}",
                    disabled: props.disabled,
                    onchange: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            fire_event(&on_change, val);
                        }
                    },
                }
            }
            if props.controls {
                span {
                    class: "el-input-number__increase",
                    role: "button",
                    onclick: move |_| {
                        if !props.disabled {
                            let new_val = (props.model_value + props.step).min(props.max);
                            fire_event(&on_change, new_val);
                        }
                    },
                    i { class: "el-icon-plus" }
                }
            }
        }
    }
}
