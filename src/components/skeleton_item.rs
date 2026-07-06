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
    let mut class_names = vec!["el-skeleton__item".to_string()];
    class_names.push(format!("el-skeleton__{}", props.variant));

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
        }
    }
}
