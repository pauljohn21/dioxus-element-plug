use dioxus::prelude::*;

// Alert and message CSS class constants
pub const ALERT: &str = "el-alert";
pub const ALERT_SUCCESS: &str = "el-alert--success";
pub const ALERT_INFO: &str = "el-alert--info";
pub const ALERT_WARNING: &str = "el-alert--warning";
pub const ALERT_ERROR: &str = "el-alert--error";
pub const ALERT_DISMISSIBLE: &str = "is-dismissible";
pub const ALERT_CLOSE: &str = "el-alert__close-btn";
pub const ALERT_ICON: &str = "el-alert__icon";
pub const ALERT_DESCRIPTION: &str = "el-alert__description";
pub const NOTIFICATION: &str = "el-notification";
pub const MESSAGE: &str = "el-message";
pub const MESSAGE_BOX: &str = "el-message-box";
pub const TOOLTIP: &str = "el-tooltip";

/// Alert types matching theme-chalk styles
#[derive(Clone, PartialEq)]
pub enum AlertType {
    Success,
    Warning,
    Info,
    Error,
}

impl AlertType {
    pub fn as_class(&self) -> &'static str {
        match self {
            AlertType::Success => "el-alert--success",
            AlertType::Warning => "el-alert--warning",
            AlertType::Info => "el-alert--info",
            AlertType::Error => "el-alert--error",
        }
    }
}

/// Alert props
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Alert title
    pub title: String,

    /// Alert content description
    #[props(default)]
    pub description: Option<String>,

    /// Alert type/style
    #[props(default = AlertType::Info)]
    pub alert_type: AlertType,

    /// Whether the alert can be closed
    #[props(default = false)]
    pub closable: bool,

    /// Whether to show icon
    #[props(default = false)]
    pub show_icon: bool,

    /// Whether the alert is center aligned
    #[props(default = false)]
    pub center: bool,

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// An alert component for displaying messages
///
/// This component displays important information with different styles
/// for success, warning, info, and error states.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::alert::{Alert, AlertType};
///
/// rsx! {
///     Alert {
///         title: "Success!".to_string(),
///         description: Some("Operation completed successfully".to_string()),
///         alert_type: AlertType::Success,
///         closable: true,
///         on_close: move |_| println!("Alert closed"),
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let mut class_names = vec![ALERT];
    
    class_names.push(props.alert_type.as_class());
    
    if props.closable {
        class_names.push("is-closable");
    }
    
    if props.show_icon {
        class_names.push("with-icon");
    }
    
    if props.center {
        class_names.push("is-center");
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let icon_class = match props.alert_type {
        AlertType::Success => "el-icon-success",
        AlertType::Warning => "el-icon-warning",
        AlertType::Info => "el-icon-info",
        AlertType::Error => "el-icon-error",
    };
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "alert",
            
            if props.show_icon {
                div {
                    class: "el-alert__icon",
                    i {
                        class: "{icon_class}"
                    }
                }
            }
            
            div {
                class: "el-alert__content",
                
                h4 {
                    class: "el-alert__title",
                    "{props.title}"
                }
                
                if let Some(ref desc) = props.description {
                    p {
                        class: "el-alert__description",
                        "{desc}"
                    }
                }
            }
            
            if props.closable {
                button {
                    class: "el-alert__close-btn",
                    type: "button",
                    onclick: move |event| {
                        if let Some(handler) = props.on_close {
                            handler.call(event);
                        }
                    },
                    "×"
                }
            }
        }
    }
}

/// Callout component for important information
#[derive(Props, Clone, PartialEq)]
pub struct CalloutProps {
    /// Callout content
    pub children: Element,

    /// Callout title
    #[props(default)]
    pub title: Option<String>,

    /// Callout type
    #[props(default = AlertType::Info)]
    pub callout_type: AlertType,

    /// Whether to show icon
    #[props(default = true)]
    pub show_icon: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A callout component for highlighting important content
///
/// This component provides a more prominent way to display important
/// information with optional title and icon.
#[component]
pub fn Callout(props: CalloutProps) -> Element {
    let mut class_names = vec!["el-callout".to_string()];
    
    class_names.push(props.callout_type.as_class().to_string());
    
    if props.show_icon {
        class_names.push("with-icon".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let icon_class = match props.callout_type {
        AlertType::Success => "el-icon-check-circle",
        AlertType::Warning => "el-icon-warning-outline",
        AlertType::Info => "el-icon-info-circle",
        AlertType::Error => "el-icon-error-circle",
    };
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            
            if props.show_icon {
                i {
                    class: "{icon_class} el-callout__icon"
                }
            }
            
            if let Some(ref title_text) = props.title {
                h4 {
                    class: "el-callout__title",
                    "{title_text}"
                }
            }
            
            div {
                class: "el-callout__content",
                {props.children}
            }
        }
    }
}

/// Notification component
#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    /// Notification title
    pub title: String,

    /// Notification message
    pub message: String,

    /// Notification type
    #[props(default = AlertType::Info)]
    pub notification_type: AlertType,

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

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<>>,
}

/// A notification component for temporary messages
///
/// This component creates a temporary notification that can auto-close
/// after a specified duration.
#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let mut class_names = vec![NOTIFICATION.to_string()];
    
    class_names.push(format!("el-notification--{}", 
        match props.notification_type {
            AlertType::Success => "success",
            AlertType::Warning => "warning",
            AlertType::Info => "info",
            AlertType::Error => "error",
        }
    ));
    
    class_names.push(format!("is-{}", props.position));
    
    let class_string = class_names.join(" ");
    
    let icon_class = match props.notification_type {
        AlertType::Success => "el-icon-check-circle",
        AlertType::Warning => "el-icon-warning-outline",
        AlertType::Info => "el-icon-info-circle",
        AlertType::Error => "el-icon-error-circle",
    };
    
    rsx! {
        div {
            class: "{class_string}",
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
                    type: "button",
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

/// Tooltip component for additional information
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Tooltip content
    pub content: String,

    /// Element to wrap with tooltip
    pub children: Element,

    /// Tooltip position
    #[props(default = "top".to_string())]
    pub placement: String,

    /// Tooltip effect (dark/light)
    #[props(default = "dark".to_string())]
    pub effect: String,

    /// Tooltip trigger (hover/click/focus)
    #[props(default = "hover".to_string())]
    pub trigger: String,

    /// Whether the tooltip is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Tooltip offset
    #[props(default = 0)]
    pub offset: i32,
}

/// A tooltip component for providing additional context
///
/// This component wraps another element and shows additional information
/// when the user hovers over or clicks on it.
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    if props.disabled {
        return rsx! { {props.children} };
    }
    
    let mut class_names = vec![TOOLTIP.to_string()];
    
    class_names.push(format!("is-{}", props.effect));
    class_names.push(format!("popper--{}", props.placement));
    
    let class_string = class_names.join(" ");
    
    rsx! {
        div {
            class: "el-tooltip",
            
            {props.children}
            
            div {
                class: "{class_string}",
                style: "display: none;",
                role: "tooltip",
                
                div {
                    class: "el-tooltip__content",
                    "{props.content}"
                }
                
                div {
                    class: "popper__arrow",
                    style: "position: absolute;"
                }
            }
        }
    }
}