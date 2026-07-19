use dioxus::prelude::*;

#[cfg(feature = "icons")]
use element_icons::element::QuestionFilled;

/// Popconfirm props
#[derive(Props, Clone, PartialEq)]
pub struct PopconfirmProps {
    #[props(default)]
    pub children: Element,

    #[props(default = "Are you sure?".to_string())]
    pub title: String,

    #[props(default = "bottom".to_string())]
    pub placement: String,

    #[props(default = "Yes".to_string())]
    pub confirm_button_text: String,

    #[props(default = "No".to_string())]
    pub cancel_button_text: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub visible: bool,

    #[props(default = "#F56C6C".to_string())]
    pub icon_color: String,

    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,

    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,
}

/// Render question icon with conditional compilation
#[cfg(feature = "icons")]
fn render_question_icon(color: &str) -> Element {
    rsx! {
        QuestionFilled {
            style: Some(format!("color: {};", color)),
        }
    }
}

/// Render question icon fallback when icons feature is disabled
#[cfg(not(feature = "icons"))]
fn render_question_icon(color: &str) -> Element {
    rsx! {
        i {
            class: "el-icon-question",
            style: "color: {color};"
        }
    }
}

/// Popconfirm component for confirmation dialogs
#[component]
pub fn Popconfirm(props: PopconfirmProps) -> Element {
    if props.disabled {
        return rsx! { {props.children} };
    }

    let confirm_text = props.confirm_button_text.clone();
    let cancel_text = props.cancel_button_text.clone();

    rsx! {
        span {
            class: "el-popconfirm__trigger",
            style: "display: inline-block; position: relative;",
            {props.children}
            if props.visible {
                div {
                    class: "el-popconfirm el-popper",
                    style: "position: absolute; z-index: 3000;",
                    div {
                        class: "el-popconfirm__main",
                        {render_question_icon(&props.icon_color)}
                        span { class: "el-popconfirm__title", "{props.title}" }
                    }
                    div {
                        class: "el-popconfirm__action",
                        button {
                            class: "el-button el-button--text el-button--small",
                            onclick: move |_| {
                                if let Some(handler) = props.on_cancel {
                                    handler.call(());
                                }
                            },
                            "{cancel_text}"
                        }
                        button {
                            class: "el-button el-button--primary el-button--small",
                            onclick: move |_| {
                                if let Some(handler) = props.on_confirm {
                                    handler.call(());
                                }
                            },
                            "{confirm_text}"
                        }
                    }
                }
            }
        }
    }
}
