use dioxus::prelude::*;

/// Divider direction
#[derive(Clone, PartialEq)]
pub enum DividerDirection {
    Horizontal,
    Vertical,
}

/// Divider content position
#[derive(Clone, PartialEq)]
pub enum DividerContentPosition {
    Left,
    Center,
    Right,
}

impl DividerContentPosition {
    pub fn as_class(&self) -> &'static str {
        match self {
            DividerContentPosition::Left => "el-divider--left",
            DividerContentPosition::Center => "",
            DividerContentPosition::Right => "el-divider--right",
        }
    }
}

/// Divider props - 分割线
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// Content text displayed in divider
    #[props(default)]
    pub children: Element,

    /// Direction
    #[props(default = DividerDirection::Horizontal)]
    pub direction: DividerDirection,

    /// Content position
    #[props(default = DividerContentPosition::Center)]
    pub content_position: DividerContentPosition,

    /// Whether to show border style
    #[props(default = true)]
    pub border_style: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Divider component - dividing line
///
/// Mirrors Element Plus `el-divider`.
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let mut class_names = vec!["el-divider".to_string()];

    match props.direction {
        DividerDirection::Horizontal => {},
        DividerDirection::Vertical => class_names.push("el-divider--vertical".to_string()),
    }

    let pos_class = props.content_position.as_class();
    if !pos_class.is_empty() {
        class_names.push(pos_class.to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    match props.direction {
        DividerDirection::Horizontal => rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
                role: "separator",

                div {
                    class: "el-divider__text",
                    {props.children}
                }
            }
        },
        DividerDirection::Vertical => rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
                role: "separator",
            }
        },
    }
}
