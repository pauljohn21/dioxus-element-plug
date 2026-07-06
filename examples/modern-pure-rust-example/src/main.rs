use dioxus::prelude::*;
use dioxus_element_plug::{
    CompleteStyleManager,
    Button, ButtonVariant,
    Input, InputType, InputSize,
    Card,
    Row, Col,
    Alert, AlertType,
};

#[component]
fn App() -> Element {
    // 生成完整 CSS
    // 这行代码调用 CompleteStyleManager 来生成全局 Element Plus 样式
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        // 注入全局样式（核心步骤）
        style { "{styles}" }

        div {
            style: "padding: 24px; background-color: #f5f7fa; min-height: 100vh;",

            h1 {
                style: "margin: 0 0 24px 0; font-size: 24px; font-weight: 600; color: #303133;",
                "Modern Pure Rust Example"
            }

            // 按钮展示
            Card {
                style: "margin-bottom: 24px;",

                h3 { "Button Variants" }

                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",

                    Button {
                        variant: ButtonVariant::Primary,
                        "Primary"
                    }

                    Button {
                        variant: ButtonVariant::Success,
                        "Success"
                    }

                    Button {
                        variant: ButtonVariant::Warning,
                        "Warning"
                    }

                    Button {
                        variant: ButtonVariant::Danger,
                        "Danger"
                    }

                    Button {
                        variant: ButtonVariant::Info,
                        "Info"
                    }
                }
            }

            // 输入框展示
            Card {
                style: "margin-bottom: 24px;",

                h3 { "Input Controls" }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    Input {
                        placeholder: "Text input",
                        input_type: InputType::Text,
                        size: InputSize::Medium,
                    }

                    Input {
                        placeholder: "Password",
                        input_type: InputType::Password,
                        size: InputSize::Medium,
                    }

                    Input {
                        placeholder: "Email",
                        input_type: InputType::Email,
                        size: InputSize::Medium,
                    }
                }
            }

            // 警告提示
            Card {
                style: "margin-bottom: 24px;",

                h3 { "Alert Messages" }

                Alert {
                    title: "Success!".to_string(),
                    alert_type: AlertType::Success,
                    style: "margin-bottom: 12px;",
                }

                Alert {
                    title: "Warning!".to_string(),
                    alert_type: AlertType::Warning,
                    style: "margin-bottom: 12px;",
                }

                Alert {
                    title: "Error!".to_string(),
                    alert_type: AlertType::Error,
                    style: "margin-bottom: 12px;",
                }

                Alert {
                    title: "Info!".to_string(),
                    alert_type: AlertType::Info,
                }
            }

            // 网格布局
            Card {
                style: "margin-bottom: 24px;",

                h3 { "Grid Layout (Row/Col)" }

                Row {
                    style: "margin-bottom: 16px;",

                    Col {
                        span: 12,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px; text-align: center;",
                            "Col 12 - Full Width"
                        }
                    }
                }

                Row {
                    style: "margin-bottom: 16px;",

                    Col {
                        span: 8,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px; text-align: center;",
                            "Col 8"
                        }
                    }

                    Col {
                        span: 8,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px; text-align: center;",
                            "Col 8"
                        }
                    }

                    Col {
                        span: 8,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px; text-align: center;",
                            "Col 8"
                        }
                    }
                }

                Row {
                    Col {
                        span: 6,
                        offset: 6,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px; text-align: center;",
                            "Col 6 Offset 6"
                        }
                    }
                }
            }

            // 统计卡片
            Card {
                style: "margin-bottom: 24px;",

                h3 { "Statistics Cards" }

                Row {
                    style: "gap: 16px;",

                    Col {
                        span: 12,
                        Card {
                            style: "background: #f6ffed;",

                            h4 { "Total Users" }

                            p {
                                style: "font-size: 24px; font-weight: 600; color: #52c41a;",
                                "2,345"
                            }

                            span { "↗ 15% from last month" }
                        }
                    }

                    Col {
                        span: 12,
                        Card {
                            style: "background: #fff7e6;",

                            h4 { "Orders" }

                            p {
                                style: "font-size: 24px; font-weight: 600; color: #fa8c16;",
                                "892"
                            }

                            span { "↗ 8% from last month" }
                        }
                    }
                }
            }

            // 主题信息
            Card {
                style: 
                    "margin-bottom: 24px; background: #f9f9f9; border-left: 4px solid #1890ff;",
                
                h3 { "About This Example" }
                
                p {
                    style: "margin: 0 0 12px 0; line-height: 1.5;",
                    "This example demonstrates the modern way to use Dioxus Element Plus with pure Rust styling. "
                }
                
                ul {
                    style: "padding-left: 20px;",
                    li { "Zero external CSS dependencies" }
                    li { "Compile-time style generation" }
                    li { "Type-safe component APIs" }
                    li { "Complete Element Plus compatibility" }
                    li { "Theme customization ready" }
                }
                
                p {
                    style: "margin: 12px 0 0 0; font-style: italic; color: #666;",
                    "Check the complete documentation in the project README.md"
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}