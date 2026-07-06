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
    let mut class_names = vec!["el-affix".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
}
