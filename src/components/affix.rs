use dioxus::prelude::*;

/// Affix position variants
#[derive(Clone, PartialEq)]
pub enum AffixPosition {
    Top,
    Bottom,
}

impl AffixPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            AffixPosition::Top => "top",
            AffixPosition::Bottom => "bottom",
        }
    }
}

/// Affix props - 固定元素在页面滚动时保持在视口内
#[derive(Props, Clone, PartialEq)]
pub struct AffixProps {
    /// Affix content
    pub children: Element,

    /// Offset distance (px)
    #[props(default = 0)]
    pub offset: u32,

    /// Position of affix (top or bottom)
    #[props(default = AffixPosition::Top)]
    pub position: AffixPosition,

    /// Z-index value
    #[props(default = 100)]
    pub z_index: u32,

    /// Whether the affix is currently fixed
    #[props(default = false)]
    pub fixed: bool,

    /// Change event when fixed state changes
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Affix component - fixes content to viewport on scroll
///
/// Mirrors Element Plus `el-affix` behavior.
#[component]
pub fn Affix(props: AffixProps) -> Element {
    let mut class_names = vec!["el-affix".to_string()];

    if props.fixed {
        class_names.push("el-affix--fixed".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }

    let class_string = class_names.join(" ");

    let mut style_parts = vec![props.style.unwrap_or_default()];
    if props.fixed {
        style_parts.push(format!("z-index: {};", props.z_index));
        match props.position {
            AffixPosition::Top => style_parts.push(format!("top: {}px;", props.offset)),
            AffixPosition::Bottom => style_parts.push(format!("bottom: {}px;", props.offset)),
        }
    }
    let style_string = style_parts.join("");

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
