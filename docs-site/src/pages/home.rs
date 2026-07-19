use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::app::Route;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "home-page",

            // Hero section
            div {
                style: "text-align: center; padding: 48px 0;",

                h1 {
                    style: "font-size: 48px; font-weight: 700; margin-bottom: 16px; color: var(--el-text-color-primary);",
                    "Dioxus Element Plug"
                }

                p {
                    style: "font-size: 20px; color: var(--el-text-color-secondary); margin-bottom: 32px;",
                    "Element Plus UI components for Dioxus - Pure Rust, Zero CSS Dependencies"
                }

                div {
                    style: "display: flex; gap: 16px; justify-content: center;",

                    Link {
                        to: Route::GettingStarted {},
                        Button {
                            variant: ButtonVariant::Primary,
                            size: Some(ButtonSize::Large),
                            "Get Started →"
                        }
                    }

                    Link {
                        to: Route::Components {},
                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Large),
                            "Browse Components"
                        }
                    }
                }
            }

            // Features
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; margin-top: 48px;",

                FeatureCard {
                    icon: "🎨",
                    title: "Pure Rust Styling",
                    description: "All CSS generated in Rust - no external CSS files or CDN dependencies. Complete style system with 50+ theme fields."
                }

                FeatureCard {
                    icon: "🌙",
                    title: "Built-in Dark Mode",
                    description: "Theme::dark() and Theme::light() presets included. Easy theme switching with unified style system."
                }

                FeatureCard {
                    icon: "🔧",
                    title: "107+ Components",
                    description: "Comprehensive component library following Element Plus design. Buttons, forms, tables, navigation, and more."
                }

                FeatureCard {
                    icon: "⚡",
                    title: "Zero Runtime Overhead",
                    description: "Styles generated at compile time. No runtime CSS processing. Fast rendering with Dioxus 0.7."
                }

                FeatureCard {
                    icon: "📱",
                    title: "Web & Desktop",
                    description: "Works with Dioxus web and desktop targets. Single codebase for multiple platforms."
                }

                FeatureCard {
                    icon: "🔣",
                    title: "Element Plus Icons",
                    description: "Optional icons feature with 137+ SVG icons. Icon components match Element Plus design."
                }
            }

            // Quick example
            div {
                style: "margin-top: 48px;",

                h2 { "Quick Example" }

                Card {
                    CardHeader {
                        title: Some("A simple counter".to_string()),
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        let mut count = use_signal(|| 0);

                        Button {
                            variant: ButtonVariant::Default,
                            on_click: move |_| count.set(count() - 1),
                            "-"
                        }

                        span {
                            style: "font-size: 24px; font-weight: 600; min-width: 48px; text-align: center;",
                            "{count}"
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            on_click: move |_| count.set(count() + 1),
                            "+"
                        }
                    }
                }

                div {
                    style: "margin-top: 16px; padding: 16px; background: var(--el-fill-color-darker); border-radius: 8px; font-family: monospace; font-size: 14px; overflow-x: auto;",
                    pre {
                        r#"use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {{
    let styles = CompleteStyleManager::new()
        .generate_complete_styles();

    let mut count = use_signal(|| 0);

    rsx! {{
        style {{ "{{styles}}" }}
        Button {{
            on_click: move |_| count.set(count() + 1),
            "Count: {{count}}"
        }}
    }}
}}"#
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FeatureCardProps {
    pub icon: String,
    pub title: String,
    pub description: String,
}

#[component]
fn FeatureCard(props: FeatureCardProps) -> Element {
    rsx! {
        Card {
            shadow: CardShadow::Hover,
            CardHeader {
                title: Some(format!("{} {}", props.icon, props.title)),
            }
            p {
                style: "color: var(--el-text-color-secondary); line-height: 1.6;",
                "{props.description}"
            }
        }
    }
}
