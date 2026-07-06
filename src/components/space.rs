use dioxus::prelude::*;

/// Space direction
#[derive(Clone, PartialEq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

/// Space size
#[derive(Clone, PartialEq)]
pub enum SpaceSize {
    Small,
    Default,
    Large,
    Custom(u32),
}

impl SpaceSize {
    pub fn as_px(&self) -> u32 {
        match self {
            SpaceSize::Small => 4,
            SpaceSize::Default => 8,
            SpaceSize::Large => 12,
            SpaceSize::Custom(px) => *px,
        }
    }
}

/// Space alignment
#[derive(Clone, PartialEq)]
pub enum SpaceAlignment {
    Start,
    Center,
    End,
    Baseline,
}

impl SpaceAlignment {
    pub fn as_class(&self) -> &'static str {
        match self {
            SpaceAlignment::Start => "is-align-start",
            SpaceAlignment::Center => "is-align-center",
            SpaceAlignment::End => "is-align-end",
            SpaceAlignment::Baseline => "is-align-baseline",
        }
    }
}

/// Space props - 间距
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    /// Space content
    pub children: Element,

    /// Direction
    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,

    /// Size
    #[props(default = SpaceSize::Default)]
    pub size: SpaceSize,

    /// Alignment
    #[props(default = SpaceAlignment::Center)]
    pub alignment: SpaceAlignment,

    /// Whether to wrap items
    #[props(default = false)]
    pub wrap: bool,

    /// Whether to fill container
    #[props(default = false)]
    pub fill: bool,

    /// Separator element
    #[props(default)]
    pub spacer: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Space component - consistent spacing between elements
///
/// Mirrors Element Plus `el-space`.
#[component]
pub fn Space(props: SpaceProps) -> Element {
    let mut class_names = vec!["el-space".to_string()];

    match props.direction {
        SpaceDirection::Horizontal => class_names.push("is-horizontal".to_string()),
        SpaceDirection::Vertical => class_names.push("is-vertical".to_string()),
    }

    class_names.push(props.alignment.as_class().to_string());

    if props.wrap {
        class_names.push("is-wrap".to_string());
    }
    if props.fill {
        class_names.push("is-fill".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let gap = props.size.as_px();
    let style_string = format!("gap: {}px;{}{}", gap, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""), if props.wrap { " flex-wrap: wrap;" } else { "" });

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
