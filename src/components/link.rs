use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Link type variants
#[derive(Clone, PartialEq)]
pub enum LinkType {
    Default,
    Primary,
    Success,
    Warning,
    Info,
    Danger,
}

impl LinkType {
    pub fn as_class(&self) -> &'static str {
        match self {
            LinkType::Default => "",
            LinkType::Primary => "el-link--primary",
            LinkType::Success => "el-link--success",
            LinkType::Warning => "el-link--warning",
            LinkType::Info => "el-link--info",
            LinkType::Danger => "el-link--danger",
        }
    }
}

/// Link underline behavior
#[derive(Clone, PartialEq)]
pub enum LinkUnderline {
    Always,
    Never,
    Hover,
}

/// Link props
#[derive(Props, Clone, PartialEq)]
pub struct LinkProps {
    /// Link content
    #[props(default)]
    pub children: Element,

    /// Link type
    #[props(default = LinkType::Default)]
    pub link_type: LinkType,

    /// Whether the link is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to show underline
    #[props(default = LinkUnderline::Hover)]
    pub underline: LinkUnderline,

    /// Hyperlink href
    #[props(default)]
    pub href: Option<String>,

    /// Hyperlink target
    #[props(default = "_self".to_string())]
    pub target: String,

    /// Icon CSS class
    #[props(default)]
    pub icon: Option<String>,

    /// Click event handler
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Link component for text hyperlinks
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Link { link_type: LinkType::Primary, href: Some("https://example.com".to_string()), "Visit" }
/// }
/// ```
#[component]
pub fn Link(props: LinkProps) -> Element {
    let mut underline_class = "";
    match props.underline {
        LinkUnderline::Always => underline_class = "is-underline",
        LinkUnderline::Never => underline_class = "is-never-underline",
        LinkUnderline::Hover => {}
    }

    let class_string = ClassBuilder::new("el-link")
        .add_class(props.link_type.as_class())
        .add_class(underline_class)
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let href = props.href.clone().unwrap_or_default();
    let on_click = props.on_click;

    rsx! {
        a {
            class: "{class_string}",
            style: "{style_string}",
            href: if props.disabled { "" } else { "{href}" },
            target: "{props.target}",
            onclick: move |event| {
                if props.disabled {
                    return;
                }
                fire_event(&on_click, event);
            },
            if let Some(ref icon) = props.icon {
                i { class: "{icon} el-link__icon" }
            }
            {props.children}
        }
    }
}
