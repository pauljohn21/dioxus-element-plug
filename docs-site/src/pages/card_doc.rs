use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::props_table::{PropInfo, PropsTable};
use crate::components::code_block::CodeBlock;
use crate::components::demo_container::DemoContainer;

#[component]
pub fn CardDocPage() -> Element {
    let mut accordion_expanded = use_signal(|| false);

    rsx! {
        div {
            h1 { "Card" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Container component with header, body, and shadow options."
            }

            // Basic example
            h2 { "Basic Usage" }

            DemoContainer {
                title: "Basic Card",
                description: Some("Card with header and content".to_string()),

                Card {
                    CardHeader {
                        title: Some("Card Title".to_string()),
                    }
                    div {
                        p { "This is the card body content." }
                        p { "Cards can contain any content you want." }
                    }
                }
            }

            CodeBlock {
                code: r#"Card {
    CardHeader {
        title: Some("Card Title".to_string()),
    }
    div {
        p { "This is the card body content." }
    }
}""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Shadows
            h2 { "Shadows" }

            DemoContainer {
                title: "Card Shadows",
                description: Some("Always, hover, or never show shadow".to_string()),

                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",

                    Card {
                        shadow: CardShadow::Always,
                        style: Some("width: 200px;".to_string()),
                        CardHeader { title: Some("Always".to_string()) }
                        "This card always has shadow"
                    }

                    Card {
                        shadow: CardShadow::Hover,
                        style: Some("width: 200px;".to_string()),
                        CardHeader { title: Some("Hover".to_string()) }
                        "Shadow on hover"
                    }

                    Card {
                        shadow: CardShadow::Never,
                        style: Some("width: 200px;".to_string()),
                        CardHeader { title: Some("Never".to_string()) }
                        "No shadow"
                    }
                }
            }

            // Accordion
            h2 { "Accordion" }

            DemoContainer {
                title: "Accordion Card",
                description: Some("Collapsible card with accordion behavior".to_string()),

                Card {
                    accordion: true,
                    expanded: accordion_expanded(),
                    on_toggle: move |expanded| accordion_expanded.set(expanded),
                    CardHeader {
                        title: Some("Accordion Card".to_string()),
                    }
                    div {
                        p { "This content can be collapsed/expanded." }
                        p { "Click the header to toggle." }
                    }
                }
            }

            CodeBlock {
                code: r#"let mut expanded = use_signal(|| false);

rsx! {
    Card {
        accordion: true,
        expanded: expanded(),
        on_toggle: move |e| expanded.set(e),
        CardHeader {
            title: Some("Accordion".to_string()),
        }
        "Collapsible content"
    }
}""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Props table
            PropsTable {
                props: vec![
                    PropInfo::new("shadow", "CardShadow", "Always", "Shadow behavior (Always, Hover, Never)"),
                    PropInfo::new("body_style", "Option<String>", "None", "Custom body styles"),
                    PropInfo::new("accordion", "bool", "false", "Enable accordion behavior"),
                    PropInfo::new("expanded", "bool", "true", "Accordion expanded state"),
                    PropInfo::new("on_toggle", "Option<EventHandler<bool>>", "None", "Accordion toggle handler"),
                ],
            }
        }
    }
}
