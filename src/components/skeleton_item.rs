use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// SkeletonItem props
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonItemProps {
    /// Item variant (text, circle, rect, etc.)
    #[props(default = "text".to_string())]
    pub variant: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// SkeletonItem component for individual skeleton placeholder
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     SkeletonItem { variant: "circle".to_string() }
/// }
/// ```
#[component]
pub fn SkeletonItem(props: SkeletonItemProps) -> Element {
    let variant_class = format!("el-skeleton__{}", props.variant);
    let class_string = ClassBuilder::new("el-skeleton__item")
        .add_class(&variant_class)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
        }
    }
}
