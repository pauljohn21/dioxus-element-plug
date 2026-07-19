use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DemoContainerProps {
    pub title: String,
    pub description: Option<String>,
    pub children: Element,
}

#[component]
pub fn DemoContainer(props: DemoContainerProps) -> Element {
    rsx! {
        div {
            class: "demo-container",
            style: "margin: 24px 0; border: 1px solid var(--el-border-color); border-radius: 8px; overflow: hidden;",

            // Header
            div {
                style: "padding: 16px; background: var(--el-fill-color-light); border-bottom: 1px solid var(--el-border-color);",

                h4 {
                    style: "margin: 0 0 8px 0; font-size: 16px; color: var(--el-text-color-primary);",
                    "{props.title}"
                }

                if let Some(desc) = props.description {
                    p {
                        style: "margin: 0; font-size: 14px; color: var(--el-text-color-secondary);",
                        "{desc}"
                    }
                }
            }

            // Demo content
            div {
                style: "padding: 24px; background: var(--el-bg-color);",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct VariantSelectorProps<T: Clone + PartialEq + 'static> {
    pub options: Vec<(String, T)>,
    pub value: T,
    pub on_change: EventHandler<T>,
    pub label: String,
}

#[component]
pub fn VariantSelector<T: Clone + PartialEq + 'static>(props: VariantSelectorProps<T>) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",

            span {
                style: "font-size: 14px; color: var(--el-text-color-regular); font-weight: 500;",
                "{props.label}:"
            }

            div {
                style: "display: flex; gap: 8px;",

                for (label, value) in props.options.iter() {
                    Button {
                        variant: if props.value == *value {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Default
                        },
                        size: Some(ButtonSize::Small),
                        on_click: {
                            let value = value.clone();
                            move |_| props.on_change.call(value.clone())
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}
