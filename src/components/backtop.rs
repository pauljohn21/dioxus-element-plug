use dioxus::prelude::*;

/// Backtop props - 回到顶部按钮
#[derive(Props, Clone, PartialEq)]
pub struct BacktopProps {
    /// Backtop content (custom icon/text)
    #[props(default)]
    pub children: Element,

    /// Visibility height (scroll threshold in px)
    #[props(default = 200)]
    pub visibility_height: u32,

    /// Right distance (px)
    #[props(default = 40)]
    pub right: u32,

    /// Bottom distance (px)
    #[props(default = 40)]
    pub bottom: u32,

    /// Whether visible (controlled)
    #[props(default = false)]
    pub visible: bool,

    /// Click event handler
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Backtop component - scroll-to-top button
///
/// Mirrors Element Plus `el-backtop`.
#[component]
pub fn Backtop(props: BacktopProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let mut class_names = vec!["el-backtop".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let style_parts = vec![
        props.style.unwrap_or_default(),
        format!("right: {}px;", props.right),
        format!("bottom: {}px;", props.bottom),
    ];
    let style_string = style_parts.join("");

    let on_click = props.on_click;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            onclick: move |e: MouseEvent| {
                if let Some(handler) = on_click.as_ref() {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}
