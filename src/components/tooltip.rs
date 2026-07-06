use dioxus::prelude::*;

/// Tooltip placement
#[derive(Clone, PartialEq)]
pub enum TooltipPlacement {
    Top, TopStart, TopEnd,
    Bottom, BottomStart, BottomEnd,
    Left, LeftStart, LeftEnd,
    Right, RightStart, RightEnd,
}

impl TooltipPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            TooltipPlacement::Top => "top",
            TooltipPlacement::TopStart => "top-start",
            TooltipPlacement::TopEnd => "top-end",
            TooltipPlacement::Bottom => "bottom",
            TooltipPlacement::BottomStart => "bottom-start",
            TooltipPlacement::BottomEnd => "bottom-end",
            TooltipPlacement::Left => "left",
            TooltipPlacement::LeftStart => "left-start",
            TooltipPlacement::LeftEnd => "left-end",
            TooltipPlacement::Right => "right",
            TooltipPlacement::RightStart => "right-start",
            TooltipPlacement::RightEnd => "right-end",
        }
    }
}

/// Tooltip props
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    #[props(default)]
    pub children: Element,

    /// Tooltip content
    pub content: String,

    /// Tooltip placement
    #[props(default = TooltipPlacement::Top)]
    pub placement: TooltipPlacement,

    /// Tooltip effect (dark/light)
    #[props(default = "dark".to_string())]
    pub effect: String,

    /// Whether the tooltip is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the tooltip is always visible
    #[props(default = false)]
    pub visible: bool,

    /// Show delay in ms
    #[props(default = 0)]
    pub show_delay: u32,

    /// Hide delay in ms
    #[props(default = 0)]
    pub hide_delay: u32,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Tooltip component for hover information
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    if props.disabled {
        return rsx! { {props.children} };
    }

    rsx! {
        span {
            class: "el-tooltip__trigger",
            style: "display: inline-block; position: relative;",
            {props.children}
            if props.visible {
                span {
                    class: "el-tooltip__popper is-{props.effect}",
                    style: "position: absolute; z-index: 3000; transform: translateX(-50%); top: -100%; left: 50%;",
                    span { class: "el-tooltip__content", "{props.content}" }
                }
            }
        }
    }
}
