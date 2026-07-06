use dioxus::prelude::*;

/// Popover props
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub title: Option<String>,

    #[props(default)]
    pub content: Option<String>,

    #[props(default = "bottom".to_string())]
    pub placement: String,

    #[props(default = "hover".to_string())]
    pub trigger: String,

    #[props(default = "dark".to_string())]
    pub effect: String,

    #[props(default = 200)]
    pub width: u32,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub visible: bool,

    #[props(default)]
    pub class: Option<String>,
}

/// Popover component for rich content tooltips
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    if props.disabled {
        return rsx! { {props.children} };
    }

    rsx! {
        span {
            class: "el-popover__trigger",
            style: "display: inline-block; position: relative;",
            {props.children}
            if props.visible {
                div {
                    class: "el-popover el-popper is-{props.effect}",
                    style: "position: absolute; z-index: 3000; width: {props.width}px;",
                    if let Some(ref title) = props.title {
                        div { class: "el-popover__title", "{title}" }
                    }
                    if let Some(ref content) = props.content {
                        div { class: "el-popover__content", "{content}" }
                    }
                }
            }
        }
    }
}
