use dioxus::prelude::*;

/// Breadcrumb separator
#[derive(Clone, PartialEq)]
pub enum BreadcrumbSeparator {
    Slash,
    Arrow,
    Custom(String),
}

impl BreadcrumbSeparator {
    pub fn as_str(&self) -> String {
        match self {
            BreadcrumbSeparator::Slash => "/".to_string(),
            BreadcrumbSeparator::Arrow => "›".to_string(),
            BreadcrumbSeparator::Custom(s) => s.clone(),
        }
    }
}

/// Breadcrumb props
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(default)]
    pub children: Element,

    #[props(default = BreadcrumbSeparator::Slash)]
    pub separator: BreadcrumbSeparator,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Breadcrumb component for navigation paths
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let mut class_names = vec!["el-breadcrumb".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            role: "navigation",
            aria_label: "Breadcrumb",
            {props.children}
        }
    }
}

/// BreadcrumbItem props
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    #[props(default)]
    pub children: Option<Element>,

    #[props(default)]
    pub to: Option<String>,

    #[props(default = false)]
    pub replace: bool,

    #[props(default = "/".to_string())]
    pub separator: String,

    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    #[props(default)]
    pub class: Option<String>,
}

/// BreadcrumbItem component for individual breadcrumb items
#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    rsx! {
        span {
            class: "el-breadcrumb__item",
            if let Some(ref to) = props.to {
                a {
                    class: "el-breadcrumb__inner is-link",
                    href: "{to}",
                    onclick: move |e| {
                        if let Some(handler) = props.on_click {
                            handler.call(e);
                        }
                    },
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
                "{props.separator}"
            }
        }
    }
}
