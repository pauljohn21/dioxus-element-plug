use dioxus::prelude::*;

/// Backtop props
#[derive(Props, Clone, PartialEq)]
pub struct BacktopProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Target element to scroll back to
    #[props(default)]
    pub target: Option<String>,

    /// Visibility height
    #[props(default = 200)]
    pub visibility_height: u32,

    /// Right position
    #[props(default = 40)]
    pub right: u32,

    /// Bottom position
    #[props(default = 40)]
    pub bottom: u32,

    #[props(default)]
    pub on_click: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,
}

/// Backtop component for scroll-to-top button
#[component]
pub fn Backtop(props: BacktopProps) -> Element {
    let mut class_names = vec!["el-backtop".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: "position: fixed; right: {props.right}px; bottom: {props.bottom}px; z-index: 1000; cursor: pointer;",
            onclick: move |_| {
                if let Some(handler) = props.on_click {
                    handler.call(());
                }
            },
            if props.children.is_some() {
                {props.children}
            } else {
                i { class: "el-icon-caret-top" }
            }
        }
    }
}
