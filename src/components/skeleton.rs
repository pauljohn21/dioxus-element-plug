use dioxus::prelude::*;

/// Skeleton variant
#[derive(Clone, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rect,
    Image,
    Caption,
    H1,
    H3,
    TextCustom { width: String },
}

/// Skeleton props - 骨架屏
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Skeleton content (actual content when loaded)
    #[props(default)]
    pub children: Element,

    /// Whether loading
    #[props(default = true)]
    pub loading: bool,

    /// Number of rows
    #[props(default = 3)]
    pub rows: u32,

    /// Whether animated
    #[props(default = true)]
    pub animated: bool,

    /// Variant
    #[props(default = SkeletonVariant::Text)]
    pub variant: SkeletonVariant,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton component - placeholder while content loads
///
/// Mirrors Element Plus `el-skeleton`.
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    if !props.loading {
        return rsx! { {props.children} };
    }

    let mut class_names = vec!["el-skeleton".to_string()];
    if props.animated {
        class_names.push("is-animated".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            for i in 0..props.rows {
                div {
                    class: "el-skeleton__item el-skeleton__paragraph",
                    style: "width: {60 + (i * 10) % 30}%; height: 16px; background: #f2f3f5; border-radius: 4px; margin-bottom: 16px;",
                }
            }
        }
    }
}
