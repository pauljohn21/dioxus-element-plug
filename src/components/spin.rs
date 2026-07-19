use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Spin props
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Whether the spinner is visible
    #[props(default = true)]
    pub spinning: bool,

    /// Spin size
    #[props(default = "default".to_string())]
    pub size: String,

    /// Spin tip text
    #[props(default)]
    pub tip: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Spin component for loading states
#[component]
pub fn Spin(props: SpinProps) -> Element {
    let size_class = format!("el-spin--{}", props.size);
    let class_string = ClassBuilder::new("el-spin")
        .add_if("is-spinning", props.spinning)
        .add_class(&size_class)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            if props.spinning {
                div {
                    class: "el-spin__container",
                    div {
                        class: "el-spin__indicator",
                        i { class: "el-icon-loading" }
                        if let Some(ref tip) = props.tip {
                            span { class: "el-spin__text", "{tip}" }
                        }
                    }
                }
            }
            {props.children}
        }
    }
}
