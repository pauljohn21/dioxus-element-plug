use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CodeExampleProps {
    pub code: String,
}

#[component]
pub fn CodeExample(props: CodeExampleProps) -> Element {
    let mut show_code = use_signal(|| false);

    rsx! {
        div {
            style: "margin: 16px 0;",

            // 切换按钮
            div {
                style: "text-align: center; padding: 8px; border-top: 1px solid var(--el-border-color); background: var(--el-fill-color-light); cursor: pointer;",
                onclick: move |_| show_code.set(!show_code()),

                span {
                    style: "font-size: 13px; color: var(--el-text-color-secondary);",
                    if show_code() { "隐藏代码 ↑" } else { "显示代码 ↓" }
                }
            }

            // 代码区域
            if show_code() {
                div {
                    style: "border-top: 1px solid var(--el-border-color);",
                    pre {
                        style: "margin: 0; padding: 20px; background: var(--el-fill-color-darker); overflow-x: auto;",
                        code {
                            style: "font-family: 'Menlo', 'Monaco', monospace; font-size: 13px; line-height: 1.6; color: var(--el-text-color-primary); white-space: pre;",
                            "{props.code}"
                        }
                    }
                }
            }
        }
    }
}
