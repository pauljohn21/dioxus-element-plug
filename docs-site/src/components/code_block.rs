use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CodeBlockProps {
    pub code: String,
    pub language: Option<String>,
}

#[component]
pub fn CodeBlock(props: CodeBlockProps) -> Element {
    let mut copied = use_signal(|| false);
    let lang = props.language.clone().unwrap_or_else(|| "rust".to_string());

    let copy_code = {
        let code = props.code.clone();
        move || {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                let window = web_sys::window().unwrap();
                let navigator = window.navigator();
                let clipboard = navigator.clipboard().unwrap();
                let _ = clipboard.write_text(&code);
            }
            copied.set(true);
            let timeout = gloo_timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            });
            timeout.forget();
        }
    };

    rsx! {
        div {
            class: "code-block",
            style: "margin: 16px 0; border: 1px solid var(--el-border-color); border-radius: 8px; overflow: hidden;",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 16px; background: var(--el-fill-color-light); border-bottom: 1px solid var(--el-border-color);",

                span {
                    style: "font-size: 12px; color: var(--el-text-color-secondary); text-transform: uppercase;",
                    "{lang}"
                }

                Button {
                    variant: ButtonVariant::Text,
                    size: Some(ButtonSize::Mini),
                    on_click: move |_| copy_code(),
                    if copied() {
                        "✓ Copied!"
                    } else {
                        "📋 Copy"
                    }
                }
            }

            // Code
            pre {
                style: "margin: 0; padding: 16px; background: var(--el-fill-color-darker); overflow-x: auto;",
                code {
                    style: "font-family: 'Fira Code', 'Monaco', 'Consolas', monospace; font-size: 14px; line-height: 1.6; color: var(--el-text-color-primary); white-space: pre;",
                    "{props.code}"
                }
            }
        }
    }
}
