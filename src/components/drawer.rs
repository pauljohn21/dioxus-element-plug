use crate::components::common::{fire_event, style_str};
use dioxus::prelude::*;

/// Drawer direction
#[derive(Clone, PartialEq)]
pub enum DrawerDirection {
    Rtl,
    Ltr,
    Ttb,
    Btt,
}

impl DrawerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            DrawerDirection::Rtl => "rtl",
            DrawerDirection::Ltr => "ltr",
            DrawerDirection::Ttb => "ttb",
            DrawerDirection::Btt => "btt",
        }
    }
}

/// Drawer props
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub title: Option<String>,

    #[props(default = false)]
    pub visible: bool,

    #[props(default = DrawerDirection::Rtl)]
    pub direction: DrawerDirection,

    #[props(default = "30%".to_string())]
    pub size: String,

    #[props(default = true)]
    pub modal: bool,

    #[props(default = true)]
    pub show_close: bool,

    #[props(default = true)]
    pub close_on_click_modal: bool,

    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Drawer component for slide-out panels
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let style_str_val = style_str(&props.style);
    let drawer_style = match props.direction {
        DrawerDirection::Rtl => format!(
            "width: {}; right: 0; top: 0; bottom: 0; {}",
            props.size, style_str_val
        ),
        DrawerDirection::Ltr => format!(
            "width: {}; left: 0; top: 0; bottom: 0; {}",
            props.size, style_str_val
        ),
        DrawerDirection::Ttb => format!(
            "height: {}; top: 0; left: 0; right: 0; {}",
            props.size, style_str_val
        ),
        DrawerDirection::Btt => format!(
            "height: {}; bottom: 0; left: 0; right: 0; {}",
            props.size, style_str_val
        ),
    };
    let on_close = props.on_close;

    rsx! {
        div {
            class: "el-overlay",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2000;",
            onclick: move |_| {
                if props.close_on_click_modal {
                    fire_event(&on_close, ());
                }
            },
            div {
                class: "el-drawer",
                style: "position: absolute; background: var(--el-bg-color); {drawer_style}",
                onclick: move |e| e.stop_propagation(),
                if let Some(ref title) = props.title {
                    header {
                        class: "el-drawer__header",
                        span { class: "el-drawer__title", "{title}" }
                        if props.show_close {
                            button {
                                class: "el-drawer__close-btn",
                                onclick: move |_| {
                                    fire_event(&on_close, ());
                                },
                                "×"
                            }
                        }
                    }
                }
                div {
                    class: "el-drawer__body",
                    {props.children}
                }
            }
        }
    }
}
