use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::props_table::{PropInfo, PropsTable};
use crate::components::code_block::CodeBlock;
use crate::components::demo_container::{DemoContainer, VariantSelector};

#[component]
pub fn ButtonDocPage() -> Element {
    let mut variant = use_signal(|| ButtonVariant::Default);
    let mut size = use_signal(|| ButtonSize::Medium);
    let mut loading = use_signal(|| false);
    let mut disabled = use_signal(|| false);
    let mut round = use_signal(|| false);
    let mut circle = use_signal(|| false);

    let variant_options = vec![
        ("Default".to_string(), ButtonVariant::Default),
        ("Primary".to_string(), ButtonVariant::Primary),
        ("Success".to_string(), ButtonVariant::Success),
        ("Warning".to_string(), ButtonVariant::Warning),
        ("Danger".to_string(), ButtonVariant::Danger),
        ("Info".to_string(), ButtonVariant::Info),
    ];

    let size_options = vec![
        ("Large".to_string(), ButtonSize::Large),
        ("Medium".to_string(), ButtonSize::Medium),
        ("Small".to_string(), ButtonSize::Small),
        ("Mini".to_string(), ButtonSize::Mini),
    ];

    let current_variant = variant();
    let current_size = size();

    rsx! {
        div {
            h1 { "Button" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Commonly used button with various styles and states."
            }

            // Interactive demo
            DemoContainer {
                title: "Interactive Demo",
                description: Some("Configure the button using the controls below".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    VariantSelector {
                        options: variant_options,
                        value: current_variant,
                        on_change: move |v| variant.set(v),
                        label: "Variant",
                    }

                    VariantSelector {
                        options: size_options,
                        value: current_size,
                        on_change: move |s| size.set(s),
                        label: "Size",
                    }

                    div {
                        style: "display: flex; gap: 16px; flex-wrap: wrap;",

                        Checkbox {
                            model_value: loading(),
                            on_change: move |v| loading.set(v),
                            "Loading"
                        }

                        Checkbox {
                            model_value: disabled(),
                            on_change: move |v| disabled.set(v),
                            "Disabled"
                        }

                        Checkbox {
                            model_value: round(),
                            on_change: move |v| round.set(v),
                            "Round"
                        }

                        Checkbox {
                            model_value: circle(),
                            on_change: move |v| circle.set(v),
                            "Circle"
                        }
                    }

                    div {
                        style: "padding: 24px; background: var(--el-fill-color-light); border-radius: 8px; display: flex; justify-content: center;",

                        Button {
                            variant: current_variant,
                            size: Some(current_size),
                            loading: loading(),
                            disabled: disabled(),
                            round: round(),
                            circle: circle(),
                            on_click: move |_| {},
                            if circle() { "✓" } else { "Button" }
                        }
                    }
                }
            }

            // Basic example
            h2 { "Basic Usage" }

            DemoContainer {
                title: "Basic Buttons",
                description: Some("Different button variants".to_string()),

                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",

                    Button { variant: ButtonVariant::Default, "Default" }
                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Success, "Success" }
                    Button { variant: ButtonVariant::Warning, "Warning" }
                    Button { variant: ButtonVariant::Danger, "Danger" }
                    Button { variant: ButtonVariant::Info, "Info" }
                }
            }

            CodeBlock {
                code: r#"Button { variant: ButtonVariant::Primary, "Primary" }""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Sizes
            h2 { "Sizes" }

            DemoContainer {
                title: "Button Sizes",
                description: Some("Buttons can be large, medium, small, or mini".to_string()),

                div {
                    style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap;",

                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "Large" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Medium), "Medium" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), "Small" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Mini), "Mini" }
                }
            }

            // States
            h2 { "States" }

            DemoContainer {
                title: "Button States",
                description: Some("Loading and disabled states".to_string()),

                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",

                    Button { variant: ButtonVariant::Primary, loading: true, "Loading" }
                    Button { variant: ButtonVariant::Primary, disabled: true, "Disabled" }
                    Button { variant: ButtonVariant::Primary, round: true, "Round" }
                    Button { variant: ButtonVariant::Primary, circle: true, "✓" }
                }
            }

            // Props table
            PropsTable {
                props: vec![
                    PropInfo::new("variant", "ButtonVariant", "Default", "Button style variant"),
                    PropInfo::new("size", "Option<ButtonSize>", "None", "Button size"),
                    PropInfo::new("disabled", "bool", "false", "Disable the button"),
                    PropInfo::new("loading", "bool", "false", "Show loading spinner"),
                    PropInfo::new("round", "bool", "false", "Round button corners"),
                    PropInfo::new("circle", "bool", "false", "Circular button"),
                    PropInfo::new("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click event handler"),
                ],
            }
        }
    }
}
