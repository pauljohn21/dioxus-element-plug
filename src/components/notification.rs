use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

// Notification CSS class constants
pub const NOTIFICATION: &str = "el-notification";

/// Notification types matching theme-chalk styles
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

/// Notification props
#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    /// Notification title
    pub title: String,

    /// Notification message
    pub message: String,

    /// Notification type
    #[props(default = NotificationType::Info)]
    pub notification_type: NotificationType,

    /// Position of the notification
    #[props(default = "top-right".to_string())]
    pub position: String,

    /// Auto close timeout in milliseconds (0 to disable)
    #[props(default = 4500)]
    pub duration: u64,

    /// Whether the notification can be closed manually
    #[props(default = true)]
    pub closable: bool,

    /// Whether to show icon
    #[props(default = true)]
    pub show_icon: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
}

/// A notification component for temporary messages
///
/// This component creates a temporary notification that can auto-close
/// after a specified duration.
#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let pos_class = format!("is-{}", props.position);
    let class_string = ClassBuilder::new("el-notification")
        .add_class(props.notification_type.as_class())
        .add_class(&pos_class)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_close = props.on_close;

    let icon_class = match props.notification_type {
        NotificationType::Success => "el-icon-check-circle",
        NotificationType::Warning => "el-icon-warning-outline",
        NotificationType::Info => "el-icon-info-circle",
        NotificationType::Error => "el-icon-error-circle",
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "alert",

            div {
                class: "el-notification__content",

                if props.show_icon {
                    i {
                        class: "{icon_class} el-notification__icon"
                    }
                }

                div {
                    class: "el-notification__group",

                    h3 {
                        class: "el-notification__title",
                        "{props.title}"
                    }

                    div {
                        class: "el-notification__message",
                        "{props.message}"
                    }
                }
            }

            if props.closable {
                button {
                    class: "el-notification__close-btn",
                    r#type: "button",
                    onclick: move |_| {
                        fire_event(&on_close, ());
                    },
                    "×"
                }
            }
        }
    }
}
