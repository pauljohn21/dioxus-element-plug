use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// Message type
#[derive(Clone, PartialEq)]
pub enum MessageType {
    Success,
    Warning,
    Info,
    Error,
}

impl MessageType {
    pub fn as_class(&self) -> &'static str {
        match self {
            MessageType::Success => "el-message--success",
            MessageType::Warning => "el-message--warning",
            MessageType::Info => "el-message--info",
            MessageType::Error => "el-message--error",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            MessageType::Success => "el-icon-success",
            MessageType::Warning => "el-icon-warning",
            MessageType::Info => "el-icon-info",
            MessageType::Error => "el-icon-error",
        }
    }
}

/// Message props
#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    /// Message text
    pub message: String,

    /// Message type
    #[props(default = MessageType::Info)]
    pub message_type: MessageType,

    /// Whether to show close button
    #[props(default = false)]
    pub closable: bool,

    /// Whether to show icon
    #[props(default = true)]
    pub show_icon: bool,

    /// Whether to center the message
    #[props(default = false)]
    pub center: bool,

    /// Display duration in ms (0 = persistent)
    #[props(default = 3000)]
    pub duration: u64,

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Message component for transient notifications
#[component]
pub fn Message(props: MessageProps) -> Element {
    let class_string = ClassBuilder::new("el-message")
        .add_class(props.message_type.as_class())
        .add_if("is-center", props.center)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_close = props.on_close;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "alert",
            if props.show_icon {
                i { class: "{props.message_type.as_icon()} el-message__icon" }
            }
            p {
                class: "el-message__content",
                "{props.message}"
            }
            if props.closable {
                button {
                    class: "el-message__closeBtn",
                    onclick: move |_| {
                        fire_event(&on_close, ());
                    },
                    "×"
                }
            }
        }
    }
}
