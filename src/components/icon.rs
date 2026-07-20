use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Icon props
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// Icon CSS class name
    #[props(default)]
    pub name: Option<String>,

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

/// Icon component for displaying icons
#[component]
pub fn Icon(props: IconProps) -> Element {
    let class_string = ClassBuilder::new("el-icon")
        .add_opt(props.name.as_ref())
        .add_opt(props.class.as_ref())
        .build();

    let mut style_parts = vec![style_str(&props.style)];
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
        }
    }
}
