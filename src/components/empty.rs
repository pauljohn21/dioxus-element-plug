use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Empty props
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    /// Image URL of empty state
    #[props(default)]
    pub image: Option<String>,

    /// Image size (width) in pixels
    #[props(default)]
    pub image_size: Option<u32>,

    /// Description text
    #[props(default = "No Data".to_string())]
    pub description: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Empty component for displaying empty states
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Empty { description: "No data found".to_string() }
/// }
/// ```
#[component]
pub fn Empty(props: EmptyProps) -> Element {
    let class_string = ClassBuilder::new("el-empty")
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let image_style = props
        .image_size
        .map(|size| format!("width: {}px", size))
        .unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-empty__image",
                style: "{image_style}",
                if let Some(ref img) = props.image {
                    img { src: "{img}" }
                } else {
                    div {
                        class: "el-empty__image-svg",
                        style: "width: 100%; height: 100%; min-height: 60px; display: flex; align-items: center; justify-content: center; color: var(--el-text-color-secondary); opacity: 0.5;",
                        "📭"
                    }
                }
            }
            p {
                class: "el-empty__description",
                "{props.description}"
            }
        }
    }
}
