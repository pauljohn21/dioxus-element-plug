use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Container 布局容器 ====================
pub fn container_page() -> Element {
    rsx! {
        div {
            h1 { "Container 布局容器" }
            p { style: "color: var(--el-text-color-secondary);", "用于布局的容器组件，方便快速搭建页面的基本结构。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "常见页面布局".to_string(),
                description: Some("使用 Container, Header, Aside, Main, Footer 组件".to_string()),
                Container {
                    direction: Some("vertical".to_string()),
                    Header {
                        div {
                            style: "background: #b3c0d1; color: #333; text-align: center; line-height: 60px;",
                            "Header"
                        }
                    }
                    Container {
                        Aside {
                            div {
                                style: "background: #d3dce6; color: #333; text-align: center; line-height: 200px;",
                                "Aside"
                            }
                        }
                        Main {
                            div {
                                style: "background: #e9eef3; color: #333; text-align: center; line-height: 160px;",
                                "Main"
                            }
                        }
                    }
                    Footer {
                        div {
                            style: "background: #b3c0d1; color: #333; text-align: center; line-height: 60px;",
                            "Footer"
                        }
                    }
                }
            }
            CodeExample {
                code: r#"Container {
    direction: Some("vertical".to_string()),
    Header {
        div {
            style: "background: #b3c0d1; text-align: center; line-height: 60px;",
            "Header"
        }
    }
    Container {
        Aside {
            div {
                style: "background: #d3dce6; text-align: center; line-height: 200px;",
                "Aside"
            }
        }
        Main {
            div {
                style: "background: #e9eef3; text-align: center; line-height: 160px;",
                "Main"
            }
        }
    }
    Footer {
        div {
            style: "background: #b3c0d1; text-align: center; line-height: 60px;",
            "Footer"
        }
    }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("direction", "子元素排列方向", "Option<String>", "None"),
                    ApiItem::new("Header height", "顶栏高度", "u32", "60"),
                    ApiItem::new("Aside width", "侧栏宽度", "u32", "200"),
                ],
            }
        }
    }
}

// ==================== Row/Col 栅格布局 ====================
pub fn row_col_page() -> Element {
    rsx! {
        div {
            h1 { "Row/Col 栅格布局" }
            p { style: "color: var(--el-text-color-secondary);", "通过基础的 24 分栏，迅速简便地创建布局。" }

            h2 { style: "margin-top: 32px;", "基础布局" }
            DemoBlock {
                title: "分栏间隔".to_string(),
                description: Some("使用 Row 的 gutter 属性设置间距".to_string()),
                Row {
                    gutter: 20,
                    Col {
                        span: 12,
                        div { style: "background: #d3dce6; border-radius: 4px; min-height: 36px;", "" }
                    }
                    Col {
                        span: 12,
                        div { style: "background: #e5e9f2; border-radius: 4px; min-height: 36px;", "" }
                    }
                }
            }
            CodeExample {
                code: r#"Row {
    gutter: 20,
    Col {
        span: 12,
        div { style: "background: #d3dce6; border-radius: 4px; min-height: 36px;", "" }
    }
    Col {
        span: 12,
        div { style: "background: #e5e9f2; border-radius: 4px; min-height: 36px;", "" }
    }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "混合布局" }
            DemoBlock {
                title: "分栏偏移".to_string(),
                description: Some("使用 Col 的 offset 属性设置偏移".to_string()),
                Row {
                    Col {
                        span: 6,
                        div { style: "background: #d3dce6; border-radius: 4px; min-height: 36px;", "" }
                    }
                    Col {
                        span: 6,
                        offset: 6,
                        div { style: "background: #e5e9f2; border-radius: 4px; min-height: 36px;", "" }
                    }
                }
            }
            CodeExample {
                code: r#"Row {
    Col {
        span: 6,
        div { style: "background: #d3dce6; border-radius: 4px; min-height: 36px;", "" }
    }
    Col {
        span: 6,
        offset: 6,
        div { style: "background: #e5e9f2; border-radius: 4px; min-height: 36px;", "" }
    }
}"#.to_string(),
            }

            ApiTable {
                title: "Row Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("gutter", "栅格间隔", "Option<u32>", "None"),
                    ApiItem::new("justify", "水平排列", "Option<String>", "None"),
                    ApiItem::new("align", "垂直排列", "Option<String>", "None"),
                ],
            }
            ApiTable {
                title: "Col Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("span", "栅格占据的列数", "Option<u32>", "None"),
                    ApiItem::new("offset", "栅格左侧间隔", "Option<u32>", "None"),
                    ApiItem::new("push", "栅格右移", "Option<u32>", "None"),
                    ApiItem::new("pull", "栅格左移", "Option<u32>", "None"),
                ],
            }
        }
    }
}
