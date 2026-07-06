use dioxus::prelude::*;
use crate::components::button::ButtonVariant;

/// Popconfirm props - 确认弹出框
#[derive(Props, Clone, PartialEq)]
pub struct PopconfirmProps {
    /// Trigger element
    pub children: Element,

    /// Confirm text
    #[props(default)]
    pub title: Option<String>,

    /// Confirm button text
    #[props(default = "确认".to_string())]
    pub confirm_button_text: String,

    /// Cancel button text
    #[props(default = "取消".to_string())]
    pub cancel_button_text: String,

    /// Confirm button type
    #[props(default = ButtonVariant::Primary)]
    pub confirm_button_type: ButtonVariant,

    /// Whether visible (controlled)
    #[props(default = false)]
    pub visible: bool,

    /// Confirm handler
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,

    /// Cancel handler
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Popconfirm component
#[component]
pub fn Popconfirm(props: PopconfirmProps) -> Element {
    let class_names = vec!["el-popconfirm".to_string()];
    let class_string = if let Some(ref c) = props.class {
        format!("{} {}", class_names.join(" "), c)
    } else {
        class_names.join(" ")
    };

    let on_confirm = props.on_confirm;
    let on_cancel = props.on_cancel;

    rsx! {
        div {
            style: "position: relative; display: inline-block;",

            {props.children}

            if props.visible {
                div {
                    class: "{class_string}",
                    style: "position: absolute; z-index: 2000;",

                    div {
                        class: "el-popconfirm__main",

                        if let Some(ref title) = props.title {
                            "{title}"
                        }
                    }

                    div {
                        class: "el-popconfirm__action",

                        button {
                            class: "el-button el-button--small",
                            onclick: move |_| {
                                if let Some(handler) = on_cancel.as_ref() {
                                    handler.call(());
                                }
                            },
                            "{props.cancel_button_text}"
                        }

                        button {
                            class: "el-button el-button--small el-button--primary",
                            onclick: move |_| {
                                if let Some(handler) = on_confirm.as_ref() {
                                    handler.call(());
                                }
                            },
                            "{props.confirm_button_text}"
                        }
                    }
                }
            }
        }
    }
}
