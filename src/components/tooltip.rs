use dioxus::prelude::*;

/// Tooltip placement
#[derive(Clone, PartialEq)]
pub enum TooltipPlacement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
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

/// Tooltip props - 文字提示
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Trigger element
    pub children: Element,

    /// Tooltip content
    #[props(default)]
    pub content: Option<String>,

    /// Placement
    #[props(default = TooltipPlacement::Top)]
    pub placement: TooltipPlacement,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Show delay (ms)
    #[props(default = 0)]
    pub show_delay: u32,

    /// Hide delay (ms)
    #[props(default = 0)]
    pub hide_delay: u32,

    /// Whether visible (controlled)
    #[props(default)]
    pub visible: Option<bool>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tooltip component
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut class_names = vec!["el-tooltip".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let show = props.visible.unwrap_or(false);
    let outer_style = format!("position: relative; display: inline-block;{}", props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        div {
            class: "{class_string}",
            style: "{outer_style}",

            {props.children}

            if show && !props.disabled {
                div {
                    class: "el-tooltip__popper is-{props.placement.as_str()}",
                    style: "position: absolute; z-index: 2000;",

                    if let Some(ref content) = props.content {
                        "{content}"
                    }
                }
            }
        }
    }
}
