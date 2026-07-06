use dioxus::prelude::*;

/// Badge type variants
#[derive(Clone, PartialEq)]
pub enum BadgeType {
    Primary,
    Success,
    Warning,
    Info,
    Danger,
}

impl BadgeType {
    pub fn as_class(&self) -> &'static str {
        match self {
            BadgeType::Primary => "el-badge__content--primary",
            BadgeType::Success => "el-badge__content--success",
            BadgeType::Warning => "el-badge__content--warning",
            BadgeType::Info => "el-badge__content--info",
            BadgeType::Danger => "el-badge__content--danger",
        }
    }
}

/// Badge props - 标记/徽章组件
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Badge content (the element the badge is attached to)
    pub children: Element,

    /// Display value
    #[props(default)]
    pub value: Option<String>,

    /// Maximum value (shows `{max}+` when exceeded)
    #[props(default = 99)]
    pub max: u32,

    /// Show a dot instead of value
    #[props(default = false)]
    pub is_dot: bool,

    /// Hidden badge
    #[props(default = false)]
    pub hidden: bool,

    /// Badge type
    #[props(default = BadgeType::Danger)]
    pub badge_type: BadgeType,

    /// Whether to show badge when value is zero
    #[props(default = true)]
    pub show_zero: bool,

    /// Custom badge background color
    #[props(default)]
    pub color: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Badge component - shows a status dot or number
///
/// Mirrors Element Plus `el-badge`.
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let mut class_names = vec!["el-badge".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    // Determine display content
    let should_show = !props.hidden && {
        if let Some(ref val) = props.value {
            if val == "0" {
                props.show_zero
            } else {
                true
            }
        } else {
            false
        }
    };

    let display_text = if props.is_dot {
        String::new()
    } else if let Some(ref val) = props.value {
        if let Ok(num) = val.parse::<u32>() {
            if num > props.max {
                format!("{}+", props.max)
            } else {
                val.clone()
            }
        } else {
            val.clone()
        }
    } else {
        String::new()
    };

    let mut content_classes = vec!["el-badge__content".to_string()];
    content_classes.push(props.badge_type.as_class().to_string());
    if props.is_dot {
        content_classes.push("is-dot".to_string());
    }
    if !should_show {
        content_classes.push("is-hidden".to_string());
    }
    let content_class = content_classes.join(" ");

    let mut content_style = String::new();
    if let Some(ref color) = props.color {
        content_style = format!("background-color: {};", color);
    }

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            {props.children}

            if should_show {
                sup {
                    class: "{content_class}",
                    style: "{content_style}",
                    "{display_text}"
                }
            }
        }
    }
}
