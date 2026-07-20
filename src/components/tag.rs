use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// Tag type variants
#[derive(Clone, PartialEq)]
pub enum TagType {
    Primary,
    Success,
    Info,
    Warning,
    Danger,
}

impl TagType {
    pub fn as_class(&self) -> &'static str {
        match self {
            TagType::Primary => "el-tag--primary",
            TagType::Success => "el-tag--success",
            TagType::Info => "el-tag--info",
            TagType::Warning => "el-tag--warning",
            TagType::Danger => "el-tag--danger",
        }
    }
}

/// Tag size variants
#[derive(Clone, PartialEq)]
pub enum TagSize {
    Large,
    Default,
    Small,
}

impl TagSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            TagSize::Large => "el-tag--large",
            TagSize::Default => "",
            TagSize::Small => "el-tag--small",
        }
    }
}

/// Tag effect variants
#[derive(Clone, PartialEq)]
pub enum TagEffect {
    Dark,
    Light,
    Plain,
}

impl TagEffect {
    pub fn as_class(&self) -> &'static str {
        match self {
            TagEffect::Dark => "el-tag--dark",
            TagEffect::Light => "el-tag--light",
            TagEffect::Plain => "el-tag--plain",
        }
    }
}

/// Tag props
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// Tag content
    #[props(default)]
    pub children: Element,

    /// Tag type
    #[props(default = TagType::Primary)]
    pub tag_type: TagType,

    /// Tag size
    #[props(default = TagSize::Default)]
    pub size: TagSize,

    /// Tag effect
    #[props(default = TagEffect::Light)]
    pub effect: TagEffect,

    /// Whether the tag is closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether the tag is rounded
    #[props(default = false)]
    pub round: bool,

    /// Whether the tag has a highlighted border
    #[props(default = false)]
    pub hit: bool,

    /// Custom background color
    #[props(default)]
    pub color: Option<String>,

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<MouseEvent>>,

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

/// Tag component for displaying status, categories or labels
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Tag { tag_type: TagType::Success, "Success" }
///     Tag { tag_type: TagType::Warning, closable: true, "Warning" }
/// }
/// ```
#[component]
pub fn Tag(props: TagProps) -> Element {
    let class_string = ClassBuilder::new("el-tag")
        .add_class(props.tag_type.as_class())
        .add_class(props.effect.as_class())
        .add_class(props.size.as_class())
        .add_if("is-round", props.round)
        .add_if("is-hit", props.hit)
        .add_opt(props.class.as_ref())
        .build();

    let mut style_string = style_str(&props.style);
    if let Some(ref color) = props.color {
        style_string = format!(
            "background-color: {}; border-color: {}; {}",
            color, color, style_string
        );
    }

    let on_click = props.on_click;
    let on_close = props.on_close;

    rsx! {
        span {
            class: "{class_string}",
            style: "{style_string}",
            onclick: move |event| {
                fire_event(&on_click, event);
            },
            {props.children}
            if props.closable {
                span {
                    class: "el-tag__close",
                    onclick: move |event| {
                        fire_event(&on_close, event);
                    },
                    "×"
                }
            }
        }
    }
}
