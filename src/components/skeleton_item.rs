use dioxus::prelude::*;

/// SkeletonItem props - 骨架屏项
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonItemProps {
    /// Variant
    #[props(default = SkeletonItemVariant::Text)]
    pub variant: SkeletonItemVariant,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton item variant
#[derive(Clone, PartialEq)]
pub enum SkeletonItemVariant {
    Text,
    Circle,
    Rect,
    Image,
    Caption,
    H1,
    H3,
}

impl SkeletonItemVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            SkeletonItemVariant::Text => "el-skeleton__paragraph",
            SkeletonItemVariant::Circle => "el-skeleton__circle",
            SkeletonItemVariant::Rect => "el-skeleton__rect",
            SkeletonItemVariant::Image => "el-skeleton__image",
            SkeletonItemVariant::Caption => "el-skeleton__caption",
            SkeletonItemVariant::H1 => "el-skeleton__h1",
            SkeletonItemVariant::H3 => "el-skeleton__h3",
        }
    }
}

/// SkeletonItem component
#[component]
pub fn SkeletonItem(props: SkeletonItemProps) -> Element {
    let mut class_names = vec!["el-skeleton__item".to_string()];
    class_names.push(props.variant.as_class().to_string());
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
        }
    }
}
