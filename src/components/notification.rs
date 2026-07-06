use dioxus::prelude::*;

/// Notification type
#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Warning,
    Info,
    Error,
}

impl NotificationType {
    pub fn as_class(&self) -> &'static str {
        match self {
            NotificationType::Success => "el-notification--success",
            NotificationType::Warning => "el-notification--warning",
            NotificationType::Info => "el-notification--info",
            NotificationType::Error => "el-notification--error",
        }
    }
}

/// Notification props - 通知
#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    /// Notification title
    #[props(default)]
    pub title: Option<String>,

    /// Notification message
    #[props(default)]
    pub message: Option<String>,

    /// Notification type
    #[props(default)]
    pub notification_type: Option<NotificationType>,

    /// Whether closable
    #[props(default = true)]
    pub closable: bool,

    /// Whether to show icon
    #[props(default = true)]
    pub show_icon: bool,

    /// Duration in ms (0 = no auto close)
    #[props(default = 4500)]
    pub duration: u32,

    /// Position
    #[props(default = "top-right".to_string())]
    pub position: String,

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

/// Notification component
#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let mut class_names = vec!["el-notification".to_string()];

    if let Some(ref nt) = props.notification_type {
        class_names.push(nt.as_class().to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_close = props.on_close;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            if props.show_icon {
                div {
                    class: "el-notification__icon",
                }
            }

            div {
                class: "el-notification__group",

                if let Some(ref title) = props.title {
                    h2 {
                        class: "el-notification__title",
                        "{title}"
                    }
                }

                if let Some(ref message) = props.message {
                    div {
                        class: "el-notification__content",
                        "{message}"
                    }
                }

                if props.closable {
                    div {
                        class: "el-notification__close-btn",
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
}
