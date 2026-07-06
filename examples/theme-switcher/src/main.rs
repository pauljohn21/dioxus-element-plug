use dioxus::prelude::*;
use dioxus_element_plug::{
    CompleteStyleManager, Theme,
    Button, ButtonVariant,
    Card,
    Input, InputType,
    Alert, AlertType,
    Row, Col,
};

/// 主题类型
#[derive(Clone, Copy, PartialEq, Debug)]
enum ThemeMode {
    Default,
    Dark,
    Green,
    Purple,
    Orange,
}

impl ThemeMode {
    fn label(&self) -> &'static str {
        match self {
            ThemeMode::Default => "Default Blue",
            ThemeMode::Dark => "Dark Mode",
            ThemeMode::Green => "Green Nature",
            ThemeMode::Purple => "Purple Dream",
            ThemeMode::Orange => "Orange Sunset",
        }
    }

    fn color_preview(&self) -> &'static str {
        match self {
            ThemeMode::Default => "#409EFF",
            ThemeMode::Dark => "#1f1f1f",
            ThemeMode::Green => "#00B96B",
            ThemeMode::Purple => "#722ED1",
            ThemeMode::Orange => "#FA8C16",
        }
    }

    fn to_theme(&self) -> Theme {
        match self {
            ThemeMode::Default => Theme::default(),

            ThemeMode::Dark => Theme {
                color_primary: "#409EFF",
                color_success: "#67C23A",
                color_warning: "#E6A23C",
                color_danger: "#F56C6C",
                color_info: "#73767A",
                color_white: "#141414",
                color_black: "#ffffff",
                color_text_primary: "#E5EAF3",
                color_text_regular: "#CFD3DC",
                color_text_secondary: "#A3A6AD",
                color_text_placeholder: "#8D9095",
                border_color_base: "#4C4D4F",
                border_color_light: "#414243",
                border_color_lighter: "#363637",
                border_color_extra_light: "#2B2B2C",
                background_color_base: "#1d1e1f",
                font_size_extra_large: "20px",
                font_size_large: "18px",
                font_size_medium: "16px",
                font_size_base: "14px",
                font_size_small: "13px",
                font_size_extra_small: "12px",
                border_radius_base: "4px",
                border_radius_small: "2px",
                border_radius_circle: "100%",
                padding_base: "12px 20px",
                all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
                fade_transition: "opacity 300ms cubic-bezier(0.23, 1, 0.32, 1)",
                border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
                color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
            },

            ThemeMode::Green => Theme {
                color_primary: "#00B96B",
                color_success: "#52C41A",
                color_warning: "#FAAD14",
                color_danger: "#FF4D4F",
                color_info: "#909399",
                color_white: "#FFFFFF",
                color_black: "#000000",
                color_text_primary: "#1f3724",
                color_text_regular: "#4a5c53",
                color_text_secondary: "#7a8a7f",
                color_text_placeholder: "#b3c2b8",
                border_color_base: "#d9eadf",
                border_color_light: "#e0eee5",
                border_color_lighter: "#ebf5ef",
                border_color_extra_light: "#f2faf6",
                background_color_base: "#f0f7f3",
                font_size_extra_large: "20px",
                font_size_large: "18px",
                font_size_medium: "16px",
                font_size_base: "14px",
                font_size_small: "13px",
                font_size_extra_small: "12px",
                border_radius_base: "6px",
                border_radius_small: "3px",
                border_radius_circle: "100%",
                padding_base: "12px 20px",
                all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
                fade_transition: "opacity 300ms cubic-bezier(0.23, 1, 0.32, 1)",
                border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
                color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
            },

            ThemeMode::Purple => Theme {
                color_primary: "#722ED1",
                color_success: "#52C41A",
                color_warning: "#FAAD14",
                color_danger: "#FF4D4F",
                color_info: "#909399",
                color_white: "#FFFFFF",
                color_black: "#000000",
                color_text_primary: "#2a1f3d",
                color_text_regular: "#534069",
                color_text_secondary: "#7d6b94",
                color_text_placeholder: "#b5a8c8",
                border_color_base: "#e0d4f0",
                border_color_light: "#e8dff2",
                border_color_lighter: "#f0eaf7",
                border_color_extra_light: "#f7f3fb",
                background_color_base: "#f5f0fa",
                font_size_extra_large: "20px",
                font_size_large: "18px",
                font_size_medium: "16px",
                font_size_base: "14px",
                font_size_small: "13px",
                font_size_extra_small: "12px",
                border_radius_base: "8px",
                border_radius_small: "4px",
                border_radius_circle: "100%",
                padding_base: "12px 20px",
                all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
                fade_transition: "opacity 300ms cubic-bezier(0.23, 1, 0.32, 1)",
                border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
                color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
            },

            ThemeMode::Orange => Theme {
                color_primary: "#FA8C16",
                color_success: "#73D13D",
                color_warning: "#FFC53D",
                color_danger: "#FF7A45",
                color_info: "#909399",
                color_white: "#FFFFFF",
                color_black: "#000000",
                color_text_primary: "#3d2814",
                color_text_regular: "#69502d",
                color_text_secondary: "#96784a",
                color_text_placeholder: "#c4ad80",
                border_color_base: "#f0d9b8",
                border_color_light: "#f5e2c6",
                border_color_lighter: "#faecd5",
                border_color_extra_light: "#fdf5e8",
                background_color_base: "#fff8ef",
                font_size_extra_large: "20px",
                font_size_large: "18px",
                font_size_medium: "16px",
                font_size_base: "14px",
                font_size_small: "13px",
                font_size_extra_small: "12px",
                border_radius_base: "6px",
                border_radius_small: "3px",
                border_radius_circle: "100%",
                padding_base: "12px 20px",
                all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
                fade_transition: "opacity 300ms cubic-bezier(0.23, 1, 0.32, 1)",
                border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
                color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
            },
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_theme = use_signal(|| ThemeMode::Default);

    let theme = current_theme.read().to_theme();

    // 提取颜色值（theme 会被 with_theme 消费）
    let primary_color = theme.color_primary.to_string();
    let success_color = theme.color_success.to_string();
    let danger_color = theme.color_danger.to_string();

    let styles = CompleteStyleManager::new()
        .with_theme(theme)
        .generate_complete_styles();

    let is_dark = *current_theme.read() == ThemeMode::Dark;
    let page_bg = if is_dark { "#1d1e1f" } else { "#f5f7fa" };
    let page_text = if is_dark { "#E5EAF3" } else { "#303133" };
    let card_bg = if is_dark { "#1f1f1f" } else { "#fff" };
    let inner_bg = if is_dark { "#363637" } else { "#f5f7fa" };
    let border_color = if is_dark { "#4C4D4F" } else { "#dcdfe6" };
    let active = *current_theme.read();

    // 预构建主题选择按钮的样式
    let theme_buttons: Vec<(ThemeMode, String, String)> = [
        ThemeMode::Default,
        ThemeMode::Dark,
        ThemeMode::Green,
        ThemeMode::Purple,
        ThemeMode::Orange,
    ]
    .iter()
    .map(|&mode| {
        let border = if active == mode {
            mode.color_preview()
        } else {
            border_color
        };
        let weight = if active == mode { "font-weight: 600;" } else { "" };
        let btn_style = format!(
            "display: flex; align-items: center; gap: 8px; padding: 10px 16px; border: 2px solid {}; border-radius: 6px; background: {}; color: {}; cursor: pointer; font-size: 14px; transition: all 0.2s; {}",
            border, card_bg, page_text, weight
        );
        let dot_style = format!(
            "display: inline-block; width: 16px; height: 16px; border-radius: 50%; background-color: {};",
            mode.color_preview()
        );
        (mode, btn_style, dot_style)
    })
    .collect();

    // 预构建颜色展示卡片样式
    let color_card_bg = format!("text-align: center; padding: 16px; background: {}; border-radius: 4px;", inner_bg);
    let primary_dot = format!("width: 40px; height: 40px; border-radius: 50%; background-color: {}; margin: 0 auto 8px;", primary_color);
    let success_dot = format!("width: 40px; height: 40px; border-radius: 50%; background-color: {}; margin: 0 auto 8px;", success_color);
    let danger_dot = format!("width: 40px; height: 40px; border-radius: 50%; background-color: {}; margin: 0 auto 8px;", danger_color);

    // 预构建底部信息卡片样式
    let info_card_style = format!("margin-bottom: 24px; border-left: 4px solid {};", primary_color);

    rsx! {
        style { "{styles}" }

        div {
            style: "padding: 24px; background-color: {page_bg}; min-height: 100vh; color: {page_text}; transition: all 0.3s ease;",

            // 标题区域
            div {
                style: "margin-bottom: 32px; text-align: center;",
                h1 {
                    style: "margin: 0 0 8px 0; font-size: 28px; font-weight: 700;",
                    "Theme Switcher Demo"
                }
                p {
                    style: "margin: 0; font-size: 14px; opacity: 0.7;",
                    "Switch between 5 built-in themes - all generated in pure Rust"
                }
            }

            // 主题选择器
            Card {
                style: "margin-bottom: 24px;",

                h3 {
                    style: "margin: 0 0 16px 0; font-size: 16px; font-weight: 600;",
                    "Select Theme"
                }

                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",

                    for (mode, btn_style, dot_style) in theme_buttons {
                        button {
                            key: "{mode:?}",
                            onclick: move |_| current_theme.set(mode),
                            style: "{btn_style}",

                            span {
                                style: "{dot_style}",
                            }
                            {mode.label()}
                        }
                    }
                }
            }

            // 当前主题信息
            Card {
                style: "margin-bottom: 24px;",

                h3 {
                    style: "margin: 0 0 16px 0; font-size: 16px; font-weight: 600;",
                    "Current Theme: {current_theme.read().label()}"
                }

                Row {
                    style: "gap: 16px;",

                    Col { span: 8,
                        div {
                            style: "{color_card_bg}",
                            p { style: "margin: 0 0 8px 0; font-size: 12px; opacity: 0.7;", "Primary Color" }
                            div { style: "{primary_dot}" }
                            code { style: "font-size: 12px;", "{primary_color}" }
                        }
                    }

                    Col { span: 8,
                        div {
                            style: "{color_card_bg}",
                            p { style: "margin: 0 0 8px 0; font-size: 12px; opacity: 0.7;", "Success Color" }
                            div { style: "{success_dot}" }
                            code { style: "font-size: 12px;", "{success_color}" }
                        }
                    }

                    Col { span: 8,
                        div {
                            style: "{color_card_bg}",
                            p { style: "margin: 0 0 8px 0; font-size: 12px; opacity: 0.7;", "Danger Color" }
                            div { style: "{danger_dot}" }
                            code { style: "font-size: 12px;", "{danger_color}" }
                        }
                    }
                }
            }

            // 按钮展示
            Card {
                style: "margin-bottom: 24px;",

                h3 { style: "margin: 0 0 16px 0; font-size: 16px; font-weight: 600;", "Buttons" }

                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",

                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Success, "Success" }
                    Button { variant: ButtonVariant::Warning, "Warning" }
                    Button { variant: ButtonVariant::Danger, "Danger" }
                    Button { variant: ButtonVariant::Info, "Info" }
                }
            }

            // 输入框展示
            Card {
                style: "margin-bottom: 24px;",

                h3 { style: "margin: 0 0 16px 0; font-size: 16px; font-weight: 600;", "Inputs" }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",

                    Input {
                        input_type: InputType::Text,
                        placeholder: "Text input",
                    }

                    Input {
                        input_type: InputType::Password,
                        placeholder: "Password input",
                    }

                    Input {
                        input_type: InputType::Email,
                        placeholder: "Email input",
                    }
                }
            }

            // Alert 展示
            Card {
                style: "margin-bottom: 24px;",

                h3 { style: "margin: 0 0 16px 0; font-size: 16px; font-weight: 600;", "Alerts" }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    Alert {
                        title: "Success message".to_string(),
                        alert_type: AlertType::Success,
                    }

                    Alert {
                        title: "Warning message".to_string(),
                        alert_type: AlertType::Warning,
                    }

                    Alert {
                        title: "Error message".to_string(),
                        alert_type: AlertType::Error,
                    }

                    Alert {
                        title: "Info message".to_string(),
                        alert_type: AlertType::Info,
                    }
                }
            }

            // 主题信息说明
            Card {
                style: "{info_card_style}",

                h3 { style: "margin: 0 0 12px 0; font-size: 16px; font-weight: 600;", "How It Works" }

                p {
                    style: "margin: 0 0 12px 0; line-height: 1.6; font-size: 14px;",
                    "This demo uses the Theme struct to customize colors, border radius, and more. "
                    "When you switch themes, CompleteStyleManager regenerates all CSS with the new theme values."
                }

                ul {
                    style: "padding-left: 20px; margin: 0; line-height: 1.8; font-size: 14px;",
                    li { "Theme struct holds all design tokens (colors, fonts, spacing)" }
                    li { "CompleteStyleManager::with_theme() applies custom theme" }
                    li { "All CSS is generated at runtime in pure Rust - zero external dependencies" }
                    li { "Theme switching is instant - just regenerate and inject styles" }
                }
            }

            // 底部
            div {
                style: "text-align: center; padding: 16px 0; font-size: 13px; opacity: 0.5;",
                "Dioxus Element Plus - Theme Switcher Example"
            }
        }
    }
}
