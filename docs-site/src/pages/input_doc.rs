use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::props_table::{PropInfo, PropsTable};
use crate::components::code_block::CodeBlock;
use crate::components::demo_container::DemoContainer;

#[component]
pub fn InputDocPage() -> Element {
    let mut text_value = use_signal(|| "Hello".to_string());
    let mut password_value = use_signal(|| "".to_string());
    let mut number_value = use_signal(|| "42".to_string());

    rsx! {
        div {
            h1 { "Input" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Input field for text, numbers, passwords, and more."
            }

            // Basic example
            h2 { "Basic Usage" }

            DemoContainer {
                title: "Text Input",
                description: Some("Basic text input with value binding".to_string()),

                div {
                    style: "max-width: 400px;",

                    Input {
                        value: Some(text_value()),
                        placeholder: Some("Enter text...".to_string()),
                        on_input: move |e| text_value.set(e.value()),
                    }

                    p {
                        style: "margin-top: 12px; color: var(--el-text-color-secondary); font-size: 14px;",
                        "Value: {text_value}"
                    }
                }
            }

            CodeBlock {
                code: r#"let mut value = use_signal(|| "".to_string());

rsx! {
    Input {
        value: Some(value()),
        placeholder: Some("Enter text...".to_string()),
        on_input: move |e| value.set(e.value()),
    }
}""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Input types
            h2 { "Input Types" }

            DemoContainer {
                title: "Different Input Types",
                description: Some("Text, password, and number inputs".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    div {
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "Text:" }
                        Input {
                            value: Some(text_value()),
                            input_type: InputType::Text,
                            on_input: move |e| text_value.set(e.value()),
                        }
                    }

                    div {
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "Password:" }
                        Input {
                            value: Some(password_value()),
                            input_type: InputType::Password,
                            placeholder: Some("Enter password...".to_string()),
                            show_password: true,
                            on_input: move |e| password_value.set(e.value()),
                        }
                    }

                    div {
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "Number:" }
                        Input {
                            value: Some(number_value()),
                            input_type: InputType::Number,
                            on_input: move |e| number_value.set(e.value()),
                        }
                    }
                }
            }

            // Sizes
            h2 { "Sizes" }

            DemoContainer {
                title: "Input Sizes",
                description: Some("Large, medium, small, and mini sizes".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    Input {
                        value: Some("Large".to_string()),
                        size: Some(InputSize::Large),
                    }
                    Input {
                        value: Some("Medium (default)".to_string()),
                        size: Some(InputSize::Medium),
                    }
                    Input {
                        value: Some("Small".to_string()),
                        size: Some(InputSize::Small),
                    }
                    Input {
                        value: Some("Mini".to_string()),
                        size: Some(InputSize::Mini),
                    }
                }
            }

            // States
            h2 { "States" }

            DemoContainer {
                title: "Input States",
                description: Some("Disabled, readonly, and error states".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    Input {
                        value: Some("Disabled".to_string()),
                        disabled: true,
                    }
                    Input {
                        value: Some("Readonly".to_string()),
                        readonly: true,
                    }
                    Input {
                        value: Some("Error state".to_string()),
                        error: true,
                    }
                    Input {
                        value: Some("Clearable".to_string()),
                        clearable: true,
                    }
                }
            }

            // Props table
            PropsTable {
                props: vec![
                    PropInfo::new("value", "Option<String>", "None", "Current input value"),
                    PropInfo::new("placeholder", "Option<String>", "None", "Placeholder text"),
                    PropInfo::new("input_type", "InputType", "Text", "Input type (Text, Password, Number, etc.)"),
                    PropInfo::new("size", "Option<InputSize>", "None", "Input size"),
                    PropInfo::new("disabled", "bool", "false", "Disable the input"),
                    PropInfo::new("readonly", "bool", "false", "Make input readonly"),
                    PropInfo::new("clearable", "bool", "false", "Show clear button"),
                    PropInfo::new("show_password", "bool", "false", "Show password toggle"),
                    PropInfo::new("error", "bool", "false", "Error state styling"),
                    PropInfo::new("on_input", "Option<EventHandler<FormEvent>>", "None", "Input event handler"),
                    PropInfo::new("on_change", "Option<EventHandler<FormEvent>>", "None", "Change event handler"),
                ],
            }
        }
    }
}
