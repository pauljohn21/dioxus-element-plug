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

/// Badge props
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Content to wrap with badge
    #[props(default)]
    pub children: Element,

    /// Display value (string or number as string)
    #[props(default)]
    pub value: Option<String>,

    /// Maximum value, shows `{max}+` when exceeded
    #[props(default = 99)]
    pub max: u32,

    /// If a little dot is displayed
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

    /// Custom dot background color
    #[props(default)]
    pub color: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Badge component for displaying notification counts or status indicators
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Badge { value: Some("5".to_string()), badge_type: BadgeType::Danger,
///         Button { "Messages" }
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    if props.hidden {
        return rsx! { {props.children} };
    }

    let mut class_names = vec!["el-badge".to_string()];
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }
    let class_string = class_names.join(" ");

    let mut content_classes = vec!["el-badge__content".to_string()];
    content_classes.push(props.badge_type.as_class().to_string());

    if props.is_dot {
        content_classes.push("is-dot".to_string());
    }

    let content_class_string = content_classes.join(" ");

    // Compute display value
    let display_value = if props.is_dot {
        String::new()
    } else {
        match &props.value {
            Some(val) => {
                if let Ok(num) = val.parse::<u32>() {
                    if num == 0 && !props.show_zero {
                        String::new()
                    } else if num > props.max {
                        format!("{}+", props.max)
                    } else {
                        val.clone()
                    }
                } else {
                    val.clone()
                }
            }
            None => String::new(),
        }
    };

    let show_content = !display_value.is_empty() || props.is_dot;

    let mut content_style = props.style.clone().unwrap_or_default();
    if let Some(ref color) = props.color {
        content_style = format!("background-color: {}; {}", color, content_style);
    }

    rsx! {
        div {
            class: "{class_string}",
            {props.children}
            if show_content {
                sup {
                    class: "{content_class_string}",
                    style: "{content_style}",
                    "{display_value}"
                }
            }
        }
    }
}
