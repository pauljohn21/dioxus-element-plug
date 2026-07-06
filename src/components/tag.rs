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

/// Tag props - 标签
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// Tag content
    pub children: Element,

    /// Tag type
    #[props(default = TagType::Primary)]
    pub tag_type: TagType,

    /// Whether closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Tag effect
    #[props(default = TagEffect::Light)]
    pub effect: TagEffect,

    /// Whether hit
    #[props(default = false)]
    pub hit: bool,

    /// Tag size
    #[props(default = TagSize::Default)]
    pub size: TagSize,

    /// Close handler
    #[props(default)]
    pub on_close: Option<EventHandler<MouseEvent>>,

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

/// Tag component - marks and categorizes content
///
/// Mirrors Element Plus `el-tag`.
#[component]
pub fn Tag(props: TagProps) -> Element {
    let mut class_names = vec!["el-tag".to_string()];

    class_names.push(props.tag_type.as_class().to_string());
    class_names.push(props.effect.as_class().to_string());

    let size_class = props.size.as_class();
    if !size_class.is_empty() {
        class_names.push(size_class.to_string());
    }

    if props.hit {
        class_names.push("is-hit".to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if props.closable {
        class_names.push("is-closable".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_close = props.on_close;
    let on_click = props.on_click;

    rsx! {
        span {
            class: "{class_string}",
            style: "{style_string}",
            onclick: move |e: MouseEvent| {
                if let Some(handler) = on_click.as_ref() {
                    handler.call(e);
                }
            },

            span {
                class: "el-tag__content",
                {props.children}
            }

            if props.closable {
                span {
                    class: "el-tag__close",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                        if let Some(handler) = on_close.as_ref() {
                            handler.call(e);
                        }
                    },
                    "×"
                }
            }
        }
    }
}
