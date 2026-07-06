use dioxus::prelude::*;

/// PageHeader props
#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Page title
    pub title: String,

    /// Page subtitle/icon text
    #[props(default)]
    pub content: Option<String>,

    /// Icon CSS class
    #[props(default)]
    pub icon: Option<String>,

    /// Back button visibility
    #[props(default = true)]
    pub show_back: bool,

    #[props(default)]
    pub on_back: Option<EventHandler<()>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// PageHeader component for page header navigation
#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    let mut class_names = vec!["el-page-header".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            if props.show_back {
                div {
                    class: "el-page-header__back",
                    onclick: move |_| {
                        if let Some(handler) = props.on_back {
                            handler.call(());
                        }
                    },
                    i { class: "el-icon-arrow-left" }
                    span { class: "el-page-header__icon", "Back" }
                }
            }
            div {
                class: "el-page-header__left",
                if let Some(ref icon) = props.icon {
                    i { class: "{icon} el-page-header__icon" }
                }
                span { class: "el-page-header__title", "{props.title}" }
            }
            div {
                class: "el-page-header__content",
                if let Some(ref content) = props.content {
                    "{content}"
                }
                {props.children}
            }
        }
    }
}
