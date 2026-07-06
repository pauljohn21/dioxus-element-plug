use dioxus::prelude::*;

/// Link type variants
#[derive(Clone, PartialEq)]
pub enum LinkType {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl LinkType {
    pub fn as_class(&self) -> &'static str {
        match self {
            LinkType::Default => "",
            LinkType::Primary => "el-link--primary",
            LinkType::Success => "el-link--success",
            LinkType::Warning => "el-link--warning",
            LinkType::Danger => "el-link--danger",
            LinkType::Info => "el-link--info",
        }
    }
}

/// Link props - 文字链接
#[derive(Props, Clone, PartialEq)]
pub struct LinkProps {
    /// Link content
    pub children: Element,

    /// Link type
    #[props(default = LinkType::Default)]
    pub link_type: LinkType,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether underline
    #[props(default = true)]
    pub underline: bool,

    /// Icon class name
    #[props(default)]
    pub icon: Option<String>,

    /// Link href
    #[props(default)]
    pub href: Option<String>,

    /// Link target
    #[props(default)]
    pub target: Option<String>,

    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Link component - text hyperlink
///
/// Mirrors Element Plus `el-link`.
#[component]
pub fn Link(props: LinkProps) -> Element {
    let mut class_names = vec!["el-link".to_string()];

    let type_class = props.link_type.as_class();
    if !type_class.is_empty() {
        class_names.push(type_class.to_string());
    }

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if !props.underline {
        class_names.push("is-underline".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_click = props.on_click;

    rsx! {
        if let Some(ref href) = props.href {
            a {
                class: "{class_string}",
                style: "{style_string}",
                href: "{href}",
                target: props.target.as_ref().map(|s| s.as_str()).unwrap_or("_blank"),
                onclick: move |e: MouseEvent| {
                    if props.disabled {
                        e.prevent_default();
                    } else if let Some(handler) = on_click.as_ref() {
                        handler.call(e);
                    }
                },
                {props.children}
            }
        } else {
            a {
                class: "{class_string}",
                style: "{style_string}",
                onclick: move |e: MouseEvent| {
                    if !props.disabled {
                        if let Some(handler) = on_click.as_ref() {
                            handler.call(e);
                        }
                    }
                },
                {props.children}
            }
        }
    }
}
