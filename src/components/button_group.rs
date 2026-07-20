use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// ButtonGroup size variants
#[derive(Clone, PartialEq)]
pub enum ButtonGroupSize {
    Large,
    Default,
    Small,
}

impl ButtonGroupSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            ButtonGroupSize::Large => "el-button-group--large",
            ButtonGroupSize::Default => "",
            ButtonGroupSize::Small => "el-button-group--small",
        }
    }
}

/// ButtonGroup direction
#[derive(Clone, PartialEq)]
pub enum ButtonGroupDirection {
    Horizontal,
    Vertical,
}

/// ButtonGroup props
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// Button group content
    #[props(default)]
    pub children: Element,

    /// Button group size
    #[props(default = ButtonGroupSize::Default)]
    pub size: ButtonGroupSize,

    /// Button group direction
    #[props(default = ButtonGroupDirection::Horizontal)]
    pub direction: ButtonGroupDirection,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// ButtonGroup component for grouping related buttons
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     ButtonGroup {
///         Button { "Prev" }
///         Button { "Next" }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let is_vertical = props.direction == ButtonGroupDirection::Vertical;
    let class_string = ClassBuilder::new("el-button-group")
        .add_class(props.size.as_class())
        .add_if("is-vertical", is_vertical)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "group",
            {props.children}
        }
    }
}
