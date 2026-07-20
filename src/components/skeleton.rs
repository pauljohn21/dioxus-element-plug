use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Skeleton props
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Real content to show when not loading
    #[props(default)]
    pub children: Option<Element>,

    /// Whether showing animation
    #[props(default = false)]
    pub animated: bool,

    /// Number of fake items to render
    #[props(default = 1)]
    pub count: u32,

    /// Number of rows per item
    #[props(default = 3)]
    pub rows: u32,

    /// Whether showing the real DOM
    #[props(default = true)]
    pub loading: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton component for displaying loading placeholders
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Skeleton { animated: true, rows: 3, loading: true }
/// }
/// ```
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    if !props.loading {
        return rsx! { {props.children} };
    }

    let class_string = ClassBuilder::new("el-skeleton")
        .add_if("is-animated", props.animated)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let items: Vec<u32> = (0..props.count).collect();
    let rows: Vec<(u32, String)> = (0..props.rows)
        .map(|idx| {
            let width = if idx == props.rows - 1 { "50%" } else { "100%" };
            (idx, width.to_string())
        })
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            for _item in items.iter() {
                div {
                    class: "el-skeleton__item",
                    for (_idx, width) in rows.iter() {
                        div {
                            class: "el-skeleton__paragraph",
                            style: "width: {width}; height: 16px; margin-top: 12px; background: var(--el-fill-color-light); border-radius: 4px;",
                        }
                    }
                }
            }
        }
    }
}
