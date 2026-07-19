use dioxus::prelude::*;
use dioxus::router::components::Link;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::Route;

// Component category modules
pub mod basic;
pub mod form;
pub mod display;
pub mod navigation;
pub mod feedback;
pub mod layout;

#[derive(Props, Clone, PartialEq)]
pub struct ComponentPageProps {
    pub name: String,
}

#[component]
pub fn ComponentPage(props: ComponentPageProps) -> Element {
    match props.name.as_str() {
        // Basic
        "button" => basic::button_page(),
        "input" => basic::input_page(),
        "input-number" => basic::input_number_page(),
        "card" => basic::card_page(),
        "tag" => basic::tag_page(),
        "badge" => basic::badge_page(),
        "link" => basic::link_page(),
        "divider" => basic::divider_page(),
        "empty" => basic::empty_page(),
        "avatar" => basic::avatar_page(),
        "button-group" => basic::button_group_page(),
        "icon" => basic::icon_page(),

        // Form
        "select" => form::select_page(),
        "switch" => form::switch_page(),
        "checkbox" => form::checkbox_page(),
        "radio" => form::radio_page(),
        "slider" => form::slider_page(),
        "rate" => form::rate_page(),
        "transfer" => form::transfer_page(),

        // Display
        "table" => display::table_page(),
        "progress" => display::progress_page(),
        "tree" => display::tree_page(),
        "image" => display::image_page(),
        "collapse" => display::collapse_page(),
        "result" => display::result_page(),
        "skeleton" => display::skeleton_page(),
        "space" => display::space_page(),

        // Navigation
        "menu" => navigation::menu_page(),
        "tabs" => navigation::tabs_page(),
        "steps" => navigation::steps_page(),
        "pagination" => navigation::pagination_page(),
        "breadcrumb" => navigation::breadcrumb_page(),
        "dropdown" => navigation::dropdown_page(),
        "page-header" => navigation::page_header_page(),
        "backtop" => navigation::backtop_page(),

        // Feedback
        "alert" => feedback::alert_page(),
        "dialog" => feedback::dialog_page(),
        "drawer" => feedback::drawer_page(),
        "tooltip" => feedback::tooltip_page(),
        "popover" => feedback::popover_page(),
        "popconfirm" => feedback::popconfirm_page(),
        "message" => feedback::message_page(),
        "notification" => feedback::notification_page(),
        "spin" => feedback::spin_page(),

        // Layout
        "container" => layout::container_page(),
        "row-col" => layout::row_col_page(),

        _ => rsx! {
            div {
                h1 { "{props.name}" }
                p { "该组件文档正在开发中..." }
            }
        }
    }
}

// 首页
#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            h1 { style: "font-size: 32px; margin-bottom: 16px;", "Dioxus Element Plug" }

            p {
                style: "font-size: 16px; color: var(--el-text-color-secondary); line-height: 1.8; margin-bottom: 32px;",
                "基于 Dioxus 的 Element Plus 组件库，使用纯 Rust 生成样式，零 CSS 依赖。"
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; margin: 32px 0;",

                Card {
                    header: Some("特性".to_string()),
                    div {
                        ul {
                            style: "line-height: 2; color: var(--el-text-color-regular);",
                            li { "107+ 组件" }
                            li { "纯 Rust 样式生成" }
                            li { "内置暗黑模式" }
                            li { "零 CSS 依赖" }
                            li { "Dioxus 0.7 支持" }
                        }
                    }
                }

                Card {
                    header: Some("快速开始".to_string()),
                    div {
                        p { style: "color: var(--el-text-color-regular); margin-bottom: 16px;", "安装依赖:" }
                        pre {
                            style: "background: var(--el-fill-color-darker); padding: 16px; border-radius: 4px; font-size: 13px; overflow-x: auto;",
                            code { "[dependencies]\ndioxus-element-plug = \"0.3\"" }
                        }
                        div {
                            style: "margin-top: 16px;",
                            Link { to: Route::QuickStart {}, "查看详细文档 →" }
                        }
                    }
                }
            }
        }
    }
}

// 安装页面
#[component]
pub fn Installation() -> Element {
    rsx! {
        div {
            h1 { "安装" }
            p { style: "color: var(--el-text-color-secondary);", "本节介绍如何在项目中安装和配置 dioxus-element-plug。" }

            h2 { style: "margin-top: 32px;", "Cargo 安装" }
            p { "推荐使用 cargo 安装:" }
            pre {
                style: "background: var(--el-fill-color-darker); padding: 16px; border-radius: 4px; overflow-x: auto;",
                code { "cargo add dioxus-element-plug" }
            }

            h2 { style: "margin-top: 32px;", "Cargo.toml" }
            p { "在 Cargo.toml 中添加:" }
            pre {
                style: "background: var(--el-fill-color-darker); padding: 16px; border-radius: 4px; overflow-x: auto;",
                        code { "[dependencies]\ndioxus = {{ version = \"0.7\", features = [\"web\"] }}\ndioxus-element-plug = \"0.3\"" }
            }
        }
    }
}

// 快速开始页面
#[component]
pub fn QuickStart() -> Element {
    rsx! {
        div {
            h1 { "快速开始" }
            p { style: "color: var(--el-text-color-secondary);", "本节介绍如何在项目中使用 dioxus-element-plug。" }

            h2 { style: "margin-top: 32px;", "引入组件库" }

            DemoBlock {
                title: "基础用法".to_string(),
                description: Some("使用 Button 组件".to_string()),

                div {
                    style: "display: flex; gap: 12px;",
                    Button { variant: ButtonVariant::Default, "默认按钮" }
                    Button { variant: ButtonVariant::Primary, "主要按钮" }
                    Button { variant: ButtonVariant::Success, "成功按钮" }
                }
            }

            CodeExample {
                code: r#"use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let styles = CompleteStyleManager::new()
        .generate_complete_styles();

    rsx! {
        style { "{styles}" }
        Button { variant: ButtonVariant::Primary, "主要按钮" }
    }
}"#.to_string(),
            }
        }
    }
}
