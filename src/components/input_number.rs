use dioxus::prelude::*;

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
    let mut class_names = vec!["el-input-number".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    class_names.push(format!("el-input-number--{}", props.size));
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            if props.controls {
                span {
                    class: "el-input-number__decrease",
                    role: "button",
                    onclick: move |_| {
                        if !props.disabled {
                            let new_val = (props.model_value - props.step).max(props.min);
                            if let Some(handler) = props.on_change {
                                handler.call(new_val);
                            }
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
                            if let Some(handler) = props.on_change {
                                handler.call(val);
                            }
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
                            if let Some(handler) = props.on_change {
                                handler.call(new_val);
                            }
                        }
                    },
                    i { class: "el-icon-plus" }
                }
            }
        }
    }
}
