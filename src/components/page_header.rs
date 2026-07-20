use crate::components::common::{fire_event, style_str, ClassBuilder};
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
    let class_string = ClassBuilder::new("el-page-header")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_back = props.on_back;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            if props.show_back {
                div {
                    class: "el-page-header__back",
                    onclick: move |_| {
                        fire_event(&on_back, ());
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
