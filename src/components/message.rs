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
}

/// Message props - 消息提示
#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    /// Message text
    #[props(default)]
    pub message: Option<String>,

    /// Message type
    #[props(default = MessageType::Info)]
    pub message_type: MessageType,

    /// Whether closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether to show icon
    #[props(default = true)]
    pub show_icon: bool,

    /// Whether centered
    #[props(default = false)]
    pub center: bool,

    /// Whether to use HTML
    #[props(default = false)]
    pub dangerously_use_html_string: bool,

    /// Duration in ms
    #[props(default = 3000)]
    pub duration: u32,

    /// Whether visible
    #[props(default = true)]
    pub visible: bool,

    /// Close handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Message component
#[component]
pub fn Message(props: MessageProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let mut class_names = vec!["el-message".to_string()];
    class_names.push(props.message_type.as_class().to_string());
    if props.center {
        class_names.push("is-center".to_string());
    }
    if props.closable {
        class_names.push("is-closable".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let on_close = props.on_close;

    rsx! {
        div {
            class: "{class_string}",

            if props.show_icon {
                i {
                    class: "el-message__icon",
                }
            }

            p {
                class: "el-message__content",
                if let Some(ref msg) = props.message {
                    "{msg}"
                }
            }

            if props.closable {
                div {
                    class: "el-message__closeBtn",
                    onclick: move |_| {
                        if let Some(handler) = on_close.as_ref() {
                            handler.call(());
                        }
                    },
                    "×"
                }
            }
        }
    }
}
