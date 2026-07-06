use dioxus::prelude::*;

/// Icon props - 图标
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// Icon content
    #[props(default)]
    pub children: Element,

    /// Icon color
    #[props(default)]
    pub color: Option<String>,

    /// Icon size
    #[props(default)]
    pub size: Option<u32>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Icon component
#[component]
pub fn Icon(props: IconProps) -> Element {
    let mut class_names = vec!["el-icon".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let mut style_parts = vec![props.style.unwrap_or_default()];
    if let Some(ref color) = props.color {
        style_parts.push(format!("color: {};", color));
    }
    if let Some(size) = props.size {
        style_parts.push(format!("font-size: {}px;", size));
    }
    let style_string = style_parts.join("");

    rsx! {
        i {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
