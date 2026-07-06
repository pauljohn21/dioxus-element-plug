use dioxus::prelude::*;

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
    let mut class_names = vec!["el-spin".to_string()];
    if props.spinning { class_names.push("is-spinning".to_string()); }
    class_names.push(format!("el-spin--{}", props.size));
    if let Some(ref c) = props.class { class_names.push(c.clone()); }
    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            style: props.style.clone().unwrap_or_default(),
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
