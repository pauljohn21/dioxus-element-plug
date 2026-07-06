use dioxus::prelude::*;

/// Breadcrumb separator props
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    /// Breadcrumb items
    pub children: Element,

    /// Separator character
    #[props(default = "/".to_string())]
    pub separator: String,

    /// Separator icon class
    #[props(default)]
    pub separator_icon: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Breadcrumb component - shows the location of the current page
///
/// Mirrors Element Plus `el-breadcrumb`.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let mut class_names = vec!["el-breadcrumb".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            aria_label: "Breadcrumb",
            role: "navigation",
            {props.children}
        }
    }
}
