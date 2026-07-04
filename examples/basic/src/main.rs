use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        // Load theme CSS
        document::Stylesheet {
            href: "/dist/theme-chalk.css"
        }

        div {
            style: "padding: 20px;",
            h1 { "Dioxus Theme Chalk Example" }

            // Button examples
            h3 { "Buttons" }
            div {
                style: "display: flex; gap: 10px; margin: 20px 0;",

                Button {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| counter += 1,
                    "Count: {counter}"
                }

                Button {
                    variant: ButtonVariant::Success,
                    "Success"
                }

                Button {
                    variant: ButtonVariant::Danger,
                    "Danger"
                }

                OutlineButton {
                    variant: ButtonVariant::Primary,
                    "Outline"
                }
            }

            // Input examples
            h3 { "Inputs" }
            div {
                style: "display: flex; flex-direction: column; gap: 10px; margin: 20px 0;",

                Input {
                    placeholder: "Enter text...",
                    size: Some(InputSize::Medium),
                }

                PasswordInput {
                    placeholder: "Password...",
                    clearable: true,
                }
            }

            // Layout example
            h3 { "Layout" }
            Container {
                Header {
                    height: 60,
                    div {
                        style: "padding: 20px; background: #409EFF; color: white;",
                        "Header"
                    }
                }
                Main {
                    Row {
                        Col {
                            span: Some(24),
                            div {
                                style: "padding: 20px; background: #f5f5f5; margin: 10px;",
                                "Content Area"
                            }
                        }
                    }
                }
                Footer {
                    height: 40,
                    div {
                        style: "padding: 10px; background: #333; color: white; text-align: center;",
                        "Footer"
                    }
                }
            }
        }
    }
}
