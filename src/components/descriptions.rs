use dioxus::prelude::*;

/// Descriptions direction
#[derive(Clone, PartialEq)]
pub enum DescriptionsDirection {
    Horizontal,
    Vertical,
}

/// Descriptions border
#[derive(Clone, PartialEq)]
pub enum DescriptionsBorder {
    None,
    Default,
    Always,
}

/// Descriptions props
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub title: Option<String>,

    #[props(default = "default".to_string())]
    pub size: String,

    #[props(default = 3)]
    pub column: u32,

    #[props(default = DescriptionsDirection::Horizontal)]
    pub direction: DescriptionsDirection,

    #[props(default = false)]
    pub border: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Descriptions component for displaying detail information
#[component]
pub fn Descriptions(props: DescriptionsProps) -> Element {
    let mut class_names = vec!["el-descriptions".to_string()];
    if props.border { class_names.push("is-bordered".to_string()); }
    class_names.push(format!("el-descriptions--{}", props.size));
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            if let Some(ref title) = props.title {
                div {
                    class: "el-descriptions__header",
                    div { class: "el-descriptions__title", "{title}" }
                }
            }
            div {
                class: "el-descriptions__body",
                {props.children}
            }
        }
    }
}
