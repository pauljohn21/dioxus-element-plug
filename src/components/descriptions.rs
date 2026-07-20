use dioxus::prelude::*;

use crate::components::common::{style_str, ClassBuilder};

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
    let size_class = format!("el-descriptions--{}", props.size);
    let class_string = ClassBuilder::new("el-descriptions")
        .add_if("is-bordered", props.border)
        .add_class(&size_class)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
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
