use dioxus::prelude::*;

/// ColorPicker props
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    #[props(default = "#409EFF".to_string())]
    pub model_value: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub show_alpha: bool,

    #[props(default = "hex".to_string())]
    pub color_format: String,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// ColorPicker component for selecting colors
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let mut class_names = vec!["el-color-picker".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let color_clone = props.model_value.clone();

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            div {
                class: "el-color-picker__trigger",
                style: "background-color: {props.model_value};",
                onclick: move |_| {
                    if !props.disabled {
                        if let Some(handler) = props.on_change {
                            handler.call(color_clone.clone());
                        }
                    }
                },
            }
        }
    }
}
