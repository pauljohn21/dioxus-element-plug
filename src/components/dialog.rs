use dioxus::prelude::*;

/// Dialog props
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// Dialog content
    #[props(default)]
    pub children: Element,

    /// Dialog title
    #[props(default)]
    pub title: Option<String>,

    /// Whether the dialog is visible
    #[props(default = false)]
    pub visible: bool,

    /// Dialog width (e.g., "50%", "500px")
    #[props(default = "50%".to_string())]
    pub width: String,

    /// Whether to show a modal overlay
    #[props(default = true)]
    pub modal: bool,

    /// Whether the dialog can be closed by clicking the mask
    #[props(default = true)]
    pub close_on_click_modal: bool,

    /// Whether the dialog can be closed by pressing ESC
    #[props(default = true)]
    pub close_on_press_escape: bool,

    /// Whether to center the dialog
    #[props(default = false)]
    pub align_center: bool,

    /// Whether the dialog is draggable
    #[props(default = false)]
    pub draggable: bool,

    /// Whether to show close button
    #[props(default = true)]
    pub show_close: bool,

    /// Top margin (e.g., "15vh")
    #[props(default = "15vh".to_string())]
    pub top: String,

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

/// Dialog component for modal dialogs
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Dialog {
///         visible: true,
///         title: Some("My Dialog".to_string()),
///         on_close: move |_| println!("Closed"),
///         "Dialog content here"
///     }
/// }
/// ```
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let mut overlay_classes = vec!["el-overlay".to_string()];
    if let Some(ref custom_class) = props.class {
        overlay_classes.push(custom_class.clone());
    }

    let mut dialog_classes = vec!["el-dialog".to_string()];
    if props.align_center {
        dialog_classes.push("el-dialog--center".to_string());
    }
    if props.draggable {
        dialog_classes.push("el-dialog--draggable".to_string());
    }

    let dialog_style = format!("width: {}; margin-top: {}; {}", props.width, props.top, props.style.clone().unwrap_or_default());

    rsx! {
        div {
            class: "{overlay_classes.join(\" \")}",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2000; overflow: auto;",
            onclick: move |_| {
                if props.close_on_click_modal {
                    if let Some(handler) = props.on_close {
                        handler.call(());
                    }
                }
            },
            div {
                class: "{dialog_classes.join(\" \")}",
                style: "{dialog_style}",
                onclick: move |e| e.stop_propagation(),
                if let Some(ref title) = props.title {
                    header {
                        class: "el-dialog__header",
                        span {
                            class: "el-dialog__title",
                            "{title}"
                        }
                        if props.show_close {
                            button {
                                class: "el-dialog__headerbtn",
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
                div {
                    class: "el-dialog__body",
                    {props.children}
                }
            }
        }
    }
}
