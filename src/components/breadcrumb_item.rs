use dioxus::prelude::*;

/// Breadcrumb item props
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    /// Item content
    pub children: Element,

    /// Target route/link
    #[props(default)]
    pub to: Option<String>,

    /// Whether to replace history
    #[props(default = false)]
    pub replace: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Breadcrumb item component
///
/// Mirrors Element Plus `el-breadcrumb-item`.
#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let mut class_names = vec!["el-breadcrumb__item".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        span {
            class: "{class_string}",

            if let Some(ref to) = props.to {
                a {
                    class: "el-breadcrumb__inner is-link",
                    href: "{to}",
                    {props.children}
                }
            } else {
                span {
                    class: "el-breadcrumb__inner",
                    {props.children}
                }
            }

            span {
                class: "el-breadcrumb__separator",
                aria_hidden: "true",
                "/"
            }
        }
    }
}
