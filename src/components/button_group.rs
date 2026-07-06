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
    let mut class_names = vec!["el-button-group".to_string()];

    let size_class = props.size.as_class();
    if !size_class.is_empty() {
        class_names.push(size_class.to_string());
    }

    match props.direction {
        ButtonGroupDirection::Horizontal => {}
        ButtonGroupDirection::Vertical => class_names.push("is-vertical".to_string()),
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "group",
            {props.children}
        }
    }
}
