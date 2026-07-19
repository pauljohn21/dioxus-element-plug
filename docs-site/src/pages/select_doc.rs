use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::props_table::{PropInfo, PropsTable};
use crate::components::code_block::CodeBlock;
use crate::components::demo_container::DemoContainer;

#[component]
pub fn SelectDocPage() -> Element {
    let mut selected_value = use_signal(|| None::<String>);

    let options = vec![
        SelectOption::new("option1", "Option 1"),
        SelectOption::new("option2", "Option 2"),
        SelectOption::new("option3", "Option 3").disabled(true),
        SelectOption::new("option4", "Option 4"),
    ];

    let selected_text = selected_value()
        .map(|v| format!("Selected: {}", v))
        .unwrap_or_else(|| "Nothing selected".to_string());

    rsx! {
        div {
            h1 { "Select" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Dropdown selection component with single select support."
            }

            // Basic example
            h2 { "Basic Usage" }

            DemoContainer {
                title: "Select Dropdown",
                description: Some("Basic select with options".to_string()),

                div {
                    style: "max-width: 400px;",

                    Select {
                        model_value: selected_value(),
                        options: options.clone(),
                        placeholder: "Please select...".to_string(),
                        on_change: move |v| selected_value.set(Some(v)),
                    }

                    p {
                        style: "margin-top: 12px; color: var(--el-text-color-secondary); font-size: 14px;",
                        "{selected_text}"
                    }
                }
            }

            CodeBlock {
                code: r#"let mut selected = use_signal(|| None::<String>);

let options = vec![
    SelectOption::new("option1", "Option 1"),
    SelectOption::new("option2", "Option 2"),
    SelectOption::new("option3", "Option 3").disabled(true),
];

rsx! {
    Select {
        model_value: selected(),
        options: options,
        placeholder: "Please select...".to_string(),
        on_change: move |v| selected.set(Some(v)),
    }
}""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Clearable
            h2 { "Clearable" }

            DemoContainer {
                title: "Clearable Select",
                description: Some("Allow clearing the selection".to_string()),

                div {
                    style: "max-width: 400px;",

                    Select {
                        model_value: selected_value(),
                        options: options.clone(),
                        placeholder: "Select and clear...".to_string(),
                        clearable: true,
                        on_change: move |v| selected_value.set(Some(v)),
                    }
                }
            }

            // Disabled
            h2 { "Disabled" }

            DemoContainer {
                title: "Disabled Select",
                description: Some("Disabled state".to_string()),

                div {
                    style: "max-width: 400px;",

                    Select {
                        model_value: None::<String>,
                        options: options.clone(),
                        placeholder: "Disabled select".to_string(),
                        disabled: true,
                        on_change: move |_| {},
                    }
                }
            }

            // Props table
            PropsTable {
                props: vec![
                    PropInfo::new("model_value", "Option<String>", "None", "Selected value"),
                    PropInfo::new("options", "Vec<SelectOption>", "vec![]", "List of options"),
                    PropInfo::new("placeholder", "String", "\"Select\"", "Placeholder text"),
                    PropInfo::new("disabled", "bool", "false", "Disable the select"),
                    PropInfo::new("clearable", "bool", "false", "Allow clearing selection"),
                    PropInfo::new("on_change", "Option<EventHandler<String>>", "None", "Change event handler"),
                ],
            }
        }
    }
}
