use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DemoBlockProps {
    pub title: String,
    pub description: Option<String>,
    pub children: Element,
}

#[component]
pub fn DemoBlock(props: DemoBlockProps) -> Element {
    rsx! {
        div {
            style: "margin: 24px 0; border: 1px solid var(--el-border-color); border-radius: 4px;",

            // 演示区域
            div {
                style: "padding: 24px; border-bottom: 1px solid var(--el-border-color); background: var(--el-bg-color);",
                {props.children}
            }

            // 说明区域
            div {
                style: "padding: 16px 24px; background: var(--el-fill-color-light);",

                h4 {
                    style: "margin: 0 0 8px 0; font-size: 14px; color: var(--el-text-color-primary);",
                    "{props.title}"
                }

                if let Some(desc) = props.description {
                    p {
                        style: "margin: 0; font-size: 13px; color: var(--el-text-color-secondary); line-height: 1.6;",
                        "{desc}"
                    }
                }
            }
        }
    }
}
