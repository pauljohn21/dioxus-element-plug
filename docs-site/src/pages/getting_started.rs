use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::code_block::CodeBlock;

#[component]
pub fn GettingStartedPage() -> Element {
    let installation_code = r#"[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-element-plug = "0.3""#;

    let basic_usage = r#"use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let styles = CompleteStyleManager::new()
        .generate_complete_styles();

    rsx! {
        style { "{styles}" }
        
        div {
            style: "padding: 24px;",
            
            h1 { "Hello!" }
            
            Button {
                variant: ButtonVariant::Primary,
                on_click: move |_| println!("Clicked!"),
                "Click Me"
            }
        }
    }
}"#;

    let dark_mode = r#"let styles = CompleteStyleManager::new()
    .with_theme(Theme::dark())
    .generate_complete_styles();"#;

    let custom_theme = r#"let custom_theme = ThemeBuilder::new()
    .primary_color("#1890ff")
    .border_radius_base("6px")
    .build();

let styles = CompleteStyleManager::new()
    .with_theme(custom_theme)
    .generate_complete_styles();"#;

    rsx! {
        div {
            h1 { "Getting Started" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Get up and running with dioxus-element-plug in minutes."
            }

            h2 { "Installation" }

            p { "Add dioxus-element-plug to your Cargo.toml:" }

            CodeBlock {
                code: installation_code.to_string(),
                language: Some("toml".to_string()),
            }

            h2 { "Basic Usage" }

            p { "Create a simple app with a Button component:" }

            CodeBlock {
                code: basic_usage.to_string(),
                language: Some("rust".to_string()),
            }

            h2 { "Dark Mode" }

            p { "Enable dark mode with Theme::dark():" }

            CodeBlock {
                code: dark_mode.to_string(),
                language: Some("rust".to_string()),
            }

            h2 { "Custom Theme" }

            p { "Customize the theme with ThemeBuilder:" }

            CodeBlock {
                code: custom_theme.to_string(),
                language: Some("rust".to_string()),
            }

            h2 { "Next" }

            ul {
                style: "line-height: 2;",
                li { "Browse the Components" }
                li { "Learn about Theming" }
                li { "Check out the examples" }
            }
        }
    }
}
