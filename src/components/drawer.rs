use dioxus::prelude::*;

/// Drawer direction
#[derive(Clone, PartialEq)]
pub enum DrawerDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl DrawerDirection {
    pub fn as_class(&self) -> &'static str {
        match self {
            DrawerDirection::Left => "ltr",
            DrawerDirection::Right => "rtl",
            DrawerDirection::Top => "ttb",
            DrawerDirection::Bottom => "btt",
        }
    }
}

/// Drawer props - 抽屉
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    /// Drawer content
    #[props(default)]
    pub children: Element,

    /// Drawer title
    #[props(default)]
    pub title: Option<String>,

    /// Whether visible
    #[props(default = false)]
    pub visible: bool,

    /// Direction
    #[props(default = DrawerDirection::Right)]
    pub direction: DrawerDirection,

    /// Drawer size
    #[props(default = "30%".to_string())]
    pub size: String,

    /// Whether modal overlay
    #[props(default = true)]
    pub modal: bool,

    /// Close on click modal
    #[props(default = true)]
    pub close_on_click_modal: bool,

    /// Show close button
    #[props(default = true)]
    pub show_close: bool,

    /// Close handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Drawer component
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let mut class_names = vec!["el-drawer".to_string()];
    class_names.push(props.direction.as_class().to_string());
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let is_horizontal = matches!(props.direction, DrawerDirection::Left | DrawerDirection::Right);
    let drawer_style = if is_horizontal {
        format!("width: {};{}", props.size, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""))
    } else {
        format!("height: {};{}", props.size, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""))
    };

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
                style: "{drawer_style}",
                onclick: |e: MouseEvent| e.stop_propagation(),

                div {
                    class: "el-drawer__header",

                    if let Some(ref title) = props.title {
                        span {
                            class: "el-drawer__title",
                            "{title}"
                        }
                    }

                    if props.show_close {
                        button {
                            class: "el-drawer__close-btn",
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
                    class: "el-drawer__body",
                    {props.children}
                }
            }
        }
    }
}
