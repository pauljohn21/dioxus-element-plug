use dioxus::prelude::*;

/// PageHeader props - 页头
#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    /// PageHeader content
    #[props(default)]
    pub children: Element,

    /// Title text
    #[props(default)]
    pub title: Option<String>,

    /// Content/subtitle text
    #[props(default)]
    pub content: Option<String>,

    /// Back icon click handler
    #[props(default)]
    pub on_back: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// PageHeader component - page header with back navigation
///
/// Mirrors Element Plus `el-page-header`.
#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    let mut class_names = vec!["el-page-header".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_back = props.on_back;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            div {
                class: "el-page-header__left",

                div {
                    class: "el-page-header__back",
                    onclick: move |e: MouseEvent| {
                        if let Some(handler) = on_back.as_ref() {
                            handler.call(e);
                        }
                    },
                    div {
                        class: "el-page-header__icon",
                        "←"
                    }
                }

                if let Some(ref title) = props.title {
                    div {
                        class: "el-page-header__title",
                        "{title}"
                    }
                }
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
