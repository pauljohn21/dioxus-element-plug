use dioxus::prelude::*;

/// Dialog props - 对话框
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// Dialog content
    #[props(default)]
    pub children: Element,

    /// Dialog title
    #[props(default)]
    pub title: Option<String>,

    /// Whether visible
    #[props(default = false)]
    pub visible: bool,

    /// Dialog width
    #[props(default = "50%".to_string())]
    pub width: String,

    /// Whether modal overlay
    #[props(default = true)]
    pub modal: bool,

    /// Close on click modal
    #[props(default = true)]
    pub close_on_click_modal: bool,

    /// Close on press escape
    #[props(default = true)]
    pub close_on_press_escape: bool,

    /// Whether center aligned
    #[props(default = false)]
    pub align_center: bool,

    /// Whether draggable
    #[props(default = false)]
    pub draggable: bool,

    /// Show close button
    #[props(default = true)]
    pub show_close: bool,

    /// Top margin
    #[props(default = "15vh".to_string())]
    pub top: String,

    /// Close handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Open handler
    #[props(default)]
    pub on_open: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Dialog component
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let mut class_names = vec!["el-dialog".to_string()];
    if props.align_center {
        class_names.push("el-dialog--center".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let dialog_style = format!("width: {}; margin-top: {};{}", props.width, props.top, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    let on_close = props.on_close;
    let close_on_click_modal = props.close_on_click_modal;

    rsx! {
        div {
            class: "el-overlay",
            onclick: move |_| {
                if close_on_click_modal {
                    if let Some(handler) = on_close.as_ref() {
                        handler.call(());
                    }
                }
            },

            div {
                class: "{class_string}",
                style: "{dialog_style}",
                onclick: |e: MouseEvent| e.stop_propagation(),

                div {
                    class: "el-dialog__header",

                    if let Some(ref title) = props.title {
                        span {
                            class: "el-dialog__title",
                            "{title}"
                        }
                    }

                    if props.show_close {
                        button {
                            class: "el-dialog__headerbtn",
                            onclick: move |_| {
                                if let Some(handler) = on_close.as_ref() {
                                    handler.call(());
                                }
                            },
                            "×"
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
