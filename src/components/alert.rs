use dioxus::prelude::*;

use crate::components::common::{fire_event, style_str, ClassBuilder};

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
    let class_string = ClassBuilder::new(ALERT)
        .add_class(props.alert_type.as_class())
        .add_if("is-closable", props.closable)
        .add_if("with-icon", props.show_icon)
        .add_if("is-center", props.center)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let icon_class = match props.alert_type {
        AlertType::Success => "el-icon-success",
        AlertType::Warning => "el-icon-warning",
        AlertType::Info => "el-icon-info",
        AlertType::Error => "el-icon-error",
    };
    let on_close = props.on_close;

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
                        fire_event(&on_close, event);
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
    let class_string = ClassBuilder::new("el-callout")
        .add_class(props.callout_type.as_class())
        .add_if("with-icon", props.show_icon)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

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
