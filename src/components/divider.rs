use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Divider direction
#[derive(Clone, PartialEq)]
pub enum DividerDirection {
    Horizontal,
    Vertical,
}

impl DividerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerDirection::Horizontal => "horizontal",
            DividerDirection::Vertical => "vertical",
        }
    }
}

/// Content position on divider
#[derive(Clone, PartialEq)]
pub enum DividerContentPosition {
    Left,
    Center,
    Right,
}

impl DividerContentPosition {
    pub fn as_class(&self) -> &'static str {
        match self {
            DividerContentPosition::Left => "is-left",
            DividerContentPosition::Center => "",
            DividerContentPosition::Right => "is-right",
        }
    }
}

/// Divider props
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// Divider content
    #[props(default)]
    pub children: Option<Element>,

    /// Direction of divider
    #[props(default = DividerDirection::Horizontal)]
    pub direction: DividerDirection,

    /// Position of content on the divider line
    #[props(default = DividerContentPosition::Center)]
    pub content_position: DividerContentPosition,

    /// Border style
    #[props(default = "solid".to_string())]
    pub border_style: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Divider component for sectioning content
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Divider { "Section Title" }
///     Divider { direction: DividerDirection::Vertical }
/// }
/// ```
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let dir_class = format!("el-divider--{}", props.direction.as_str());
    let has_content = props.children.is_some();
    let has_text = has_content && props.direction == DividerDirection::Horizontal;

    let class_string = ClassBuilder::new("el-divider")
        .add_class(&dir_class)
        .add_if(props.content_position.as_class(), has_text)
        .add_if("el-divider--text", has_text)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = format!(
        "border-top-style: {}; {}",
        props.border_style,
        style_str(&props.style)
    );

    if has_content && props.direction == DividerDirection::Horizontal {
        rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
                div {
                    class: "el-divider__text",
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
            }
        }
    }
}
