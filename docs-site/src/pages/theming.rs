use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::components::demo_container::DemoContainer;

#[component]
pub fn ThemingPage() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut custom_primary = use_signal(|| "#409EFF".to_string());

    let current_theme = if is_dark() {
        Theme::dark()
    } else {
        Theme::light()
    };

    let dark_code = r#"let theme = if is_dark {
    Theme::dark()
} else {
    Theme::light()
};

let styles = CompleteStyleManager::new()
    .with_theme(theme)
    .generate_complete_styles();"#;

    let custom_code = r#"let custom_theme = ThemeBuilder::new()
    .primary_color("#1890ff")
    .border_radius_base("6px")
    .build();

let styles = CompleteStyleManager::new()
    .with_theme(custom_theme)
    .generate_complete_styles();"#;

    rsx! {
        div {
            h1 { "Theming" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Customize the look and feel of your application."
            }

            h2 { "Dark Mode" }

            DemoContainer {
                title: "Theme Toggle Demo",
                description: Some("Click the button to toggle between light and dark themes".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| is_dark.set(!is_dark()),
                        if is_dark() { "Switch to Light" } else { "Switch to Dark" }
                    }

                    Card {
                        CardHeader {
                            title: Some("Current Theme".to_string()),
                        }
                        div {
                            p { "Primary Color: ",
                                span {
                                    style: "display: inline-block; width: 20px; height: 20px; background: {current_theme.primary_color}; border-radius: 4px; vertical-align: middle; margin-left: 8px;",
                                }
                            }
                        }
                    }
                }
            }

            CodeBlock {
                code: dark_code.to_string(),
                language: Some("rust".to_string()),
            }

            h2 { "Custom Theme" }

            DemoContainer {
                title: "Custom Primary Color",
                description: Some("Enter a hex color to customize the primary color".to_string()),

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    Input {
                        value: Some(custom_primary()),
                        placeholder: Some("Enter hex color".to_string()),
                        on_input: move |e| custom_primary.set(e.value()),
                    }

                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",

                        Button {
                            variant: ButtonVariant::Primary,
                            "Primary"
                        }
                        Button {
                            variant: ButtonVariant::Success,
                            "Success"
                        }
                    }
                }
            }

            CodeBlock {
                code: custom_code.to_string(),
                language: Some("rust".to_string()),
            }

            h2 { "Theme Options" }

            p { "The Theme struct has many customizable fields." }

            ul {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 8px; line-height: 1.8;",
                li { "primary_color" }
                li { "success_color" }
                li { "warning_color" }
                li { "danger_color" }
                li { "text_color_primary" }
                li { "border_radius_base" }
                li { "font_size_base" }
            }
        }
    }
}
