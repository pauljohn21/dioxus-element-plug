use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Loading props
#[derive(Props, Clone, PartialEq)]
pub struct LoadingProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Whether loading is active
    #[props(default = false)]
    pub loading: bool,

    /// Loading text
    #[props(default)]
    pub text: Option<String>,

    /// Whether to show a fullscreen overlay
    #[props(default = false)]
    pub fullscreen: bool,

    /// Background color
    #[props(default = "rgba(255, 255, 255, 0.9)".to_string())]
    pub background: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Loading component for displaying loading indicators
#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let class_string = ClassBuilder::new("el-loading")
        .add_if("el-loading--fullscreen", props.fullscreen)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
            if props.loading {
                div {
                    class: "el-loading__mask",
                    style: "background-color: {props.background};",
                    div {
                        class: "el-loading__spinner",
                        i { class: "el-icon-loading" }
                        if let Some(ref text) = props.text {
                            p { class: "el-loading__text", "{text}" }
                        }
                    }
                }
            }
        }
    }
}

/// LoadingDirective - standalone loading indicator
#[derive(Props, Clone, PartialEq)]
pub struct LoadingDirectiveProps {
    #[props(default = false)]
    pub fullscreen: bool,

    #[props(default)]
    pub text: Option<String>,

    #[props(default)]
    pub class: Option<String>,
}

/// LoadingDirective - standalone loading spinner
#[component]
pub fn LoadingDirective(props: LoadingDirectiveProps) -> Element {
    let class_string = ClassBuilder::new("el-loading")
        .add_if("el-loading--fullscreen", props.fullscreen)
        .add_opt(props.class.as_ref())
        .build();

    rsx! {
        div {
            class: "{class_string}",
            style: "position: relative;",
            div {
                class: "el-loading__mask",
                div {
                    class: "el-loading__spinner",
                    i { class: "el-icon-loading" }
                    if let Some(ref text) = props.text {
                        p { class: "el-loading__text", "{text}" }
                    }
                }
            }
        }
    }
}
