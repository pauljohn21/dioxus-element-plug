use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// MessageBox type
#[derive(Clone, PartialEq)]
pub enum MessageBoxType {
    Success,
    Warning,
    Info,
    Error,
}

impl MessageBoxType {
    pub fn as_class(&self) -> &'static str {
        match self {
            MessageBoxType::Success => "el-message-box--success",
            MessageBoxType::Warning => "el-message-box--warning",
            MessageBoxType::Info => "el-message-box--info",
            MessageBoxType::Error => "el-message-box--error",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            MessageBoxType::Success => "el-icon-success",
            MessageBoxType::Warning => "el-icon-warning",
            MessageBoxType::Info => "el-icon-info",
            MessageBoxType::Error => "el-icon-error",
        }
    }
}

/// MessageBox props
#[derive(Props, Clone, PartialEq)]
pub struct MessageBoxProps {
    /// Title text
    #[props(default)]
    pub title: Option<String>,

    /// Message content
    pub message: String,

    /// Message box type
    #[props(default = MessageBoxType::Info)]
    pub box_type: MessageBoxType,

    /// Whether to show close button
    #[props(default = true)]
    pub show_close: bool,

    /// Whether to show confirm button
    #[props(default = true)]
    pub show_confirm_button: bool,

    /// Whether to show cancel button
    #[props(default = false)]
    pub show_cancel_button: bool,

    /// Confirm button text
    #[props(default = "OK".to_string())]
    pub confirm_button_text: String,

    /// Cancel button text
    #[props(default = "Cancel".to_string())]
    pub cancel_button_text: String,

    /// Whether the message box is visible
    #[props(default = true)]
    pub visible: bool,

    /// Confirm event handler
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,

    /// Cancel event handler
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// MessageBox component for modal dialogs with messages
#[component]
pub fn MessageBox(props: MessageBoxProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let class_string = ClassBuilder::new("el-message-box")
        .add_class(props.box_type.as_class())
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_confirm = props.on_confirm;
    let on_cancel = props.on_cancel;

    rsx! {
        div {
            class: "el-overlay",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2001;",
            div {
                class: "{class_string}",
                style: "{style_string}",
                if let Some(ref title) = props.title {
                    div {
                        class: "el-message-box__header",
                        i { class: "{props.box_type.as_icon()} el-message-box__icon" }
                        span { class: "el-message-box__title", "{title}" }
                        if props.show_close {
                            button {
                                class: "el-message-box__close",
                                onclick: move |_| {
                                    fire_event(&on_cancel, ());
                                },
                                "×"
                            }
                        }
                    }
                }
                div {
                    class: "el-message-box__content",
                    p { class: "el-message-box__message", "{props.message}" }
                }
                div {
                    class: "el-message-box__btns",
                    if props.show_cancel_button {
                        button {
                            class: "el-button el-button--default",
                            onclick: move |_| {
                                fire_event(&on_cancel, ());
                            },
                            "{props.cancel_button_text}"
                        }
                    }
                    if props.show_confirm_button {
                        button {
                            class: "el-button el-button--primary",
                            onclick: move |_| {
                                fire_event(&on_confirm, ());
                            },
                            "{props.confirm_button_text}"
                        }
                    }
                }
            }
        }
    }
}
