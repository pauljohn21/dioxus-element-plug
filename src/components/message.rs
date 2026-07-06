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
    let mut class_names = vec!["el-message".to_string()];
    class_names.push(props.message_type.as_class().to_string());
    if props.center { class_names.push("is-center".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
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
                        if let Some(handler) = props.on_close {
                            handler.call(());
                        }
                    },
                    "×"
                }
            }
        }
    }
}
