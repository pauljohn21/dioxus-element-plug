use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Affix props
#[derive(Props, Clone, PartialEq)]
pub struct AffixProps {
    #[props(default)]
    pub children: Element,

    /// Target container offset top
    #[props(default = 0)]
    pub offset: u32,

    /// Affix position
    #[props(default = "top".to_string())]
    pub position: String,

    /// Target element selector
    #[props(default)]
    pub target: Option<String>,

    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Affix component for fixing elements on scroll
#[component]
pub fn Affix(props: AffixProps) -> Element {
    let class_string = ClassBuilder::new("el-affix")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
