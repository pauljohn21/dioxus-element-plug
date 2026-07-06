use dioxus::prelude::*;

/// Popover placement
#[derive(Clone, PartialEq)]
pub enum PopoverPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl PopoverPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverPlacement::Top => "top",
            PopoverPlacement::Bottom => "bottom",
            PopoverPlacement::Left => "left",
            PopoverPlacement::Right => "right",
        }
    }
}

/// Popover trigger type
#[derive(Clone, PartialEq)]
pub enum PopoverTrigger {
    Click,
    Hover,
    Focus,
    Manual,
}

/// Popover props - 气泡卡片
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Trigger element
    pub children: Element,

    /// Popover content
    #[props(default)]
    pub content: Option<String>,

    /// Popover title
    #[props(default)]
    pub title: Option<String>,

    /// Placement
    #[props(default = PopoverPlacement::Bottom)]
    pub placement: PopoverPlacement,

    /// Trigger type
    #[props(default = PopoverTrigger::Click)]
    pub trigger: PopoverTrigger,

    /// Width
    #[props(default = "150px".to_string())]
    pub width: String,

    /// Whether visible (controlled)
    #[props(default = false)]
    pub visible: bool,

    /// Close handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Popover component
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let mut class_names = vec!["el-popover".to_string()];
    class_names.push(format!("el-popover--{}", props.placement.as_str()));
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let outer_style = format!("position: relative; display: inline-block;{}", props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        div {
            style: "{outer_style}",

            {props.children}

            if props.visible {
                div {
                    class: "{class_string}",
                    style: "position: absolute; z-index: 2000; width: {props.width};",

                    if let Some(ref title) = props.title {
                        div {
                            class: "el-popover__title",
                            "{title}"
                        }
                    }

                    if let Some(ref content) = props.content {
                        "{content}"
                    }
                }
            }
        }
    }
}
