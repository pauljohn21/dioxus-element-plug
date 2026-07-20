use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// DescriptionsItem props
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsItemProps {
    #[props(default)]
    pub children: Element,

    /// Label text
    pub label: String,

    /// Column span (applied as colspan on content cell)
    #[props(default = 1)]
    pub span: u32,

    /// Label width
    #[props(default)]
    pub label_width: Option<String>,

    /// Additional CSS classes for content cell
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles for content cell
    #[props(default)]
    pub style: Option<String>,
}

/// DescriptionsItem component for individual description entries
#[component]
pub fn DescriptionsItem(props: DescriptionsItemProps) -> Element {
    let label_class = ClassBuilder::new("el-descriptions__cell")
        .add_class("el-descriptions__label")
        .build();

    let content_class = ClassBuilder::new("el-descriptions__cell")
        .add_class("el-descriptions__content")
        .add_opt(props.class.as_ref())
        .build();

    let content_style = style_str(&props.style);
    let label_style = props
        .label_width
        .as_ref()
        .map(|w| format!("width: {w};"))
        .unwrap_or_default();

    rsx! {
        td {
            class: "{label_class}",
            style: "{label_style}",
            "{props.label}"
        }
        td {
            class: "{content_class}",
            colspan: "{props.span}",
            style: "{content_style}",
            {props.children}
        }
    }
}
