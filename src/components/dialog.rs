use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

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

    let overlay_class = ClassBuilder::new("el-overlay")
        .add_opt(props.class.as_ref())
        .build();

    let dialog_class = ClassBuilder::new("el-dialog")
        .add_if("el-dialog--center", props.align_center)
        .add_if("el-dialog--draggable", props.draggable)
        .build();

    let dialog_style = format!("width: {}; margin-top: {}; {}", props.width, props.top, style_str(&props.style));
    let on_close = props.on_close;

    rsx! {
        div {
            class: "{overlay_class}",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2000; overflow: auto;",
            onclick: move |_| {
                if props.close_on_click_modal {
                    fire_event(&on_close, ());
                }
            },
            div {
                class: "{dialog_class}",
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
                                    fire_event(&on_close, ());
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
