use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use std::collections::HashMap;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Table 表格 ====================
pub fn table_page() -> Element {
    rsx! {
        div {
            h1 { "Table 表格" }
            p { style: "color: var(--el-text-color-secondary);", "用于展示多条结构类似的数据，可对数据进行排序、筛选、对比等操作。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "基础表格".to_string(),
                description: Some("使用 columns 和 data 属性定义表格".to_string()),
                Table {
                    columns: vec![
                        TableColumn { title: "姓名".to_string(), key: "name".to_string(), width: None, sortable: false, fixed: None },
                        TableColumn { title: "年龄".to_string(), key: "age".to_string(), width: None, sortable: true, fixed: None },
                        TableColumn { title: "地址".to_string(), key: "address".to_string(), width: None, sortable: false, fixed: None },
                    ],
                    data: vec![
                        HashMap::from([("name".to_string(), "张三".to_string()), ("age".to_string(), "30".to_string()), ("address".to_string(), "北京".to_string())]),
                        HashMap::from([("name".to_string(), "李四".to_string()), ("age".to_string(), "25".to_string()), ("address".to_string(), "上海".to_string())]),
                        HashMap::from([("name".to_string(), "王五".to_string()), ("age".to_string(), "28".to_string()), ("address".to_string(), "广州".to_string())]),
                    ],
                }
            }
            CodeExample {
                code: r#"Table {
    columns: vec![
        TableColumn { title: "姓名".to_string(), key: "name".to_string(), width: None, sortable: false, fixed: None },
        TableColumn { title: "年龄".to_string(), key: "age".to_string(), width: None, sortable: true, fixed: None },
        TableColumn { title: "地址".to_string(), key: "address".to_string(), width: None, sortable: false, fixed: None },
    ],
    data: vec![
        HashMap::from([("name".to_string(), "张三".to_string()), ("age".to_string(), "30".to_string()), ("address".to_string(), "北京".to_string())]),
        HashMap::from([("name".to_string(), "李四".to_string()), ("age".to_string(), "25".to_string()), ("address".to_string(), "上海".to_string())]),
    ],
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "带边框和斑马纹" }
            DemoBlock {
                title: "边框/斑马纹".to_string(),
                description: Some("使用 border 和 stripe 属性".to_string()),
                Table {
                    stripe: true,
                    border: true,
                    columns: vec![
                        TableColumn { title: "日期".to_string(), key: "date".to_string(), width: None, sortable: false, fixed: None },
                        TableColumn { title: "姓名".to_string(), key: "name".to_string(), width: None, sortable: false, fixed: None },
                    ],
                    data: vec![
                        HashMap::from([("date".to_string(), "2024-01-01".to_string()), ("name".to_string(), "张三".to_string())]),
                        HashMap::from([("date".to_string(), "2024-01-02".to_string()), ("name".to_string(), "李四".to_string())]),
                    ],
                }
            }
            CodeExample {
                code: r#"Table {
    stripe: true,
    border: true,
    columns: vec![
        TableColumn { title: "日期".to_string(), key: "date".to_string(), width: None, sortable: false, fixed: None },
        TableColumn { title: "姓名".to_string(), key: "name".to_string(), width: None, sortable: false, fixed: None },
    ],
    data: vec![
        HashMap::from([("date".to_string(), "2024-01-01".to_string()), ("name".to_string(), "张三".to_string())]),
        HashMap::from([("date".to_string(), "2024-01-02".to_string()), ("name".to_string(), "李四".to_string())]),
    ],
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("columns", "列定义", "Vec<TableColumn>", "required"),
                    ApiItem::new("data", "表格数据", "Vec<HashMap<String, String>>", "required"),
                    ApiItem::new("stripe", "是否斑马纹", "bool", "false"),
                    ApiItem::new("border", "是否带边框", "bool", "false"),
                    ApiItem::new("show_header", "是否显示表头", "bool", "true"),
                    ApiItem::new("on_row_click", "行点击事件", "Option<EventHandler<usize>>", "None"),
                ],
            }
        }
    }
}

// ==================== Progress 进度条 ====================
pub fn progress_page() -> Element {
    rsx! {
        div {
            h1 { "Progress 进度条" }
            p { style: "color: var(--el-text-color-secondary);", "用于展示操作进度，告知用户当前状态和预期。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "线性进度条".to_string(),
                description: Some("使用 percentage 属性设置百分比".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Progress { percentage: 30 }
                    Progress { percentage: 60 }
                    Progress { percentage: 100, status: ProgressStatus::Success }
                    Progress { percentage: 80, status: ProgressStatus::Exception }
                }
            }
            CodeExample {
                code: r#"Progress { percentage: 30 }
Progress { percentage: 60 }
Progress { percentage: 100, status: ProgressStatus::Success }
Progress { percentage: 80, status: ProgressStatus::Exception }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "环形进度条" }
            DemoBlock {
                title: "环形".to_string(),
                description: Some("使用 progress_type 属性设置为环形".to_string()),
                div {
                    style: "display: flex; gap: 20px;",
                    Progress { percentage: 75, progress_type: ProgressType::Circle }
                    Progress { percentage: 100, progress_type: ProgressType::Circle, status: ProgressStatus::Success }
                }
            }
            CodeExample {
                code: r#"Progress { percentage: 75, progress_type: ProgressType::Circle }
Progress { percentage: 100, progress_type: ProgressType::Circle, status: ProgressStatus::Success }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("percentage", "百分比", "u32", "0"),
                    ApiItem::new("progress_type", "类型", "ProgressType", "Line"),
                    ApiItem::new("status", "状态", "ProgressStatus", "Default"),
                    ApiItem::new("stroke_width", "线宽", "u32", "6"),
                    ApiItem::new("show_text", "是否显示文字", "bool", "true"),
                    ApiItem::new("indeterminate", "是否不定状态", "bool", "false"),
                ],
            }
        }
    }
}

// ==================== Tree 树形控件 ====================
pub fn tree_page() -> Element {
    rsx! {
        div {
            h1 { "Tree 树形控件" }
            p { style: "color: var(--el-text-color-secondary);", "用清晰的层级结构展示信息，可展开或折叠。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "基础树".to_string(),
                description: Some("使用 data 属性传入树形数据".to_string()),
                Tree {
                    data: vec![
                        TreeNodeData::new("一级 1")
                            .child(TreeNodeData::new("二级 1-1"))
                            .child(TreeNodeData::new("二级 1-2")),
                        TreeNodeData::new("一级 2")
                            .child(TreeNodeData::new("二级 2-1")),
                    ],
                }
            }
            CodeExample {
                code: r#"Tree {
    data: vec![
        TreeNodeData::new("一级 1")
            .child(TreeNodeData::new("二级 1-1"))
            .child(TreeNodeData::new("二级 1-2")),
        TreeNodeData::new("一级 2")
            .child(TreeNodeData::new("二级 2-1")),
    ],
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("data", "树数据", "Vec<TreeNodeData>", "[]"),
                    ApiItem::new("show_checkbox", "是否显示复选框", "bool", "false"),
                    ApiItem::new("default_expand_all", "默认展开全部", "bool", "false"),
                    ApiItem::new("on_node_click", "节点点击事件", "Option<EventHandler<String>>", "None"),
                    ApiItem::new("on_node_expand", "节点展开事件", "Option<EventHandler<(String, bool)>>", "None"),
                ],
            }
        }
    }
}

// ==================== Image 图片 ====================
pub fn image_page() -> Element {
    rsx! {
        div {
            h1 { "Image 图片" }
            p { style: "color: var(--el-text-color-secondary);", "图片容器，在保留原生 img 特性的基础上，支持懒加载、自定义占位等。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "图片".to_string(),
                description: Some("使用 src 属性设置图片地址".to_string()),
                div {
                    style: "display: flex; gap: 20px;",
                    Image {
                        src: "https://cube.elemecstatic.com/compress/tests/1ac946a8c3e872da5f35.jpg".to_string(),
                        alt: Some("示例图片".to_string()),
                        fit: ImageFit::Cover,
                    }
                }
            }
            CodeExample {
                code: r#"Image {
    src: "https://cube.elemecstatic.com/compress/tests/1ac946a8c3e872da5f35.jpg".to_string(),
    alt: Some("示例图片".to_string()),
    fit: ImageFit::Cover,
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("src", "图片地址", "String", "required"),
                    ApiItem::new("alt", "替代文本", "Option<String>", "None"),
                    ApiItem::new("fit", "适应方式", "ImageFit", "Cover"),
                    ApiItem::new("lazy", "是否懒加载", "bool", "false"),
                ],
            }
        }
    }
}

// ==================== Collapse 折叠面板 ====================
pub fn collapse_page() -> Element {
    rsx! {
        div {
            h1 { "Collapse 折叠面板" }
            p { style: "color: var(--el-text-color-secondary);", "通过折叠面板收纳内容区域。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "折叠面板".to_string(),
                description: Some("使用 Collapse 和 CollapseItem 组件".to_string()),
                Collapse {
                    active_names: vec!["1".to_string()],
                    CollapseItem { name: "1".to_string(), title: "一致性 Consistency".to_string(),
                        div { "与现实生活一致" }
                    }
                    CollapseItem { name: "2".to_string(), title: "反馈 Feedback".to_string(),
                        div { "用户通过界面交互时提供反馈" }
                    }
                }
            }
            CodeExample {
                code: r#"Collapse {
    active_names: vec!["1".to_string()],
    CollapseItem {
        name: "1".to_string(),
        title: "一致性 Consistency".to_string(),
        div { "与现实生活一致" }
    }
    CollapseItem {
        name: "2".to_string(),
        title: "反馈 Feedback".to_string(),
        div { "用户通过界面交互时提供反馈" }
    }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("active_names", "展开项", "Vec<String>", "[]"),
                    ApiItem::new("accordion", "是否手风琴模式", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<Vec<String>>>", "None"),
                ],
            }
        }
    }
}

// ==================== Result 结果页 ====================
pub fn result_page() -> Element {
    rsx! {
        div {
            h1 { "Result 结果" }
            p { style: "color: var(--el-text-color-secondary);", "用于反馈一系列操作任务的处理结果。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同状态".to_string(),
                description: Some("使用 result_type 属性定义结果类型".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    Result { result_type: ResultType::Success, title: Some("提交成功".to_string()) }
                    Result { result_type: ResultType::Warning, title: Some("警告提示".to_string()) }
                    Result { result_type: ResultType::Error, title: Some("提交失败".to_string()) }
                }
            }
            CodeExample {
                code: r#"Result { result_type: ResultType::Success, title: Some("提交成功".to_string()) }
Result { result_type: ResultType::Warning, title: Some("警告提示".to_string()) }
Result { result_type: ResultType::Error, title: Some("提交失败".to_string()) }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("result_type", "结果类型", "ResultType", "Info"),
                    ApiItem::new("title", "标题", "Option<String>", "None"),
                    ApiItem::new("sub_title", "副标题", "Option<String>", "None"),
                    ApiItem::new("icon", "图标", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Skeleton 骨架屏 ====================
pub fn skeleton_page() -> Element {
    rsx! {
        div {
            h1 { "Skeleton 骨架屏" }
            p { style: "color: var(--el-text-color-secondary);", "在需要等待加载内容的位置提供一个占位图形组合。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "骨架屏".to_string(),
                description: Some("使用 animated 属性启用动画".to_string()),
                Skeleton { animated: true, rows: 4, count: 1 }
            }
            CodeExample {
                code: r#"Skeleton { animated: true, rows: 4, count: 1 }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("animated", "是否动画", "bool", "false"),
                    ApiItem::new("count", "渲染数量", "u32", "1"),
                    ApiItem::new("rows", "行数", "u32", "3"),
                ],
            }
        }
    }
}

// ==================== Space 间距 ====================
pub fn space_page() -> Element {
    rsx! {
        div {
            h1 { "Space 间距" }
            p { style: "color: var(--el-text-color-secondary);", "提供组件之间的间距。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "水平间距".to_string(),
                description: Some("默认水平方向间距".to_string()),
                Space {
                    Button { variant: ButtonVariant::Default, "按钮1" }
                    Button { variant: ButtonVariant::Default, "按钮2" }
                    Button { variant: ButtonVariant::Default, "按钮3" }
                }
            }
            CodeExample {
                code: r#"Space {
    Button { variant: ButtonVariant::Default, "按钮1" }
    Button { variant: ButtonVariant::Default, "按钮2" }
    Button { variant: ButtonVariant::Default, "按钮3" }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "垂直方向" }
            DemoBlock {
                title: "垂直间距".to_string(),
                description: Some("使用 direction 属性设置为垂直".to_string()),
                Space {
                    direction: SpaceDirection::Vertical,
                    Button { variant: ButtonVariant::Default, "按钮1" }
                    Button { variant: ButtonVariant::Default, "按钮2" }
                }
            }
            CodeExample {
                code: r#"Space {
    direction: SpaceDirection::Vertical,
    Button { variant: ButtonVariant::Default, "按钮1" }
    Button { variant: ButtonVariant::Default, "按钮2" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("direction", "方向", "SpaceDirection", "Horizontal"),
                ],
            }
        }
    }
}
