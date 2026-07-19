use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Menu 导航菜单 ====================
pub fn menu_page() -> Element {
    rsx! {
        div {
            h1 { "Menu 导航菜单" }
            p { style: "color: var(--el-text-color-secondary);", "为页面和功能提供导航的菜单列表。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "水平菜单".to_string(),
                description: Some("使用 mode 属性设置为水平模式".to_string()),
                Menu {
                    mode: MenuMode::Horizontal,
                    MenuItem { index: "1".to_string(), "处理中心" }
                    MenuItem { index: "2".to_string(), disabled: true, "消息中心" }
                    SubMenu { index: "3".to_string(), title: Some("我的工作台".to_string()),
                        MenuItem { index: "3-1".to_string(), "选项1" }
                        MenuItem { index: "3-2".to_string(), "选项2" }
                    }
                }
            }
            CodeExample {
                code: r#"Menu {
    mode: MenuMode::Horizontal,
    MenuItem { index: "1".to_string(), "处理中心" }
    MenuItem { index: "2".to_string(), disabled: true, "消息中心" }
    SubMenu {
        index: "3".to_string(),
        title: Some("我的工作台".to_string()),
        MenuItem { index: "3-1".to_string(), "选项1" }
        MenuItem { index: "3-2".to_string(), "选项2" }
    }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "垂直菜单" }
            DemoBlock {
                title: "垂直菜单".to_string(),
                description: Some("默认垂直模式，可折叠".to_string()),
                div { style: "max-width: 200px;",
                    Menu {
                        mode: MenuMode::Vertical,
                        MenuItem { index: "1".to_string(), "首页" }
                        MenuItem { index: "2".to_string(), "用户管理" }
                        MenuItem { index: "3".to_string(), "系统设置" }
                    }
                }
            }
            CodeExample {
                code: r#"Menu {
    mode: MenuMode::Vertical,
    MenuItem { index: "1".to_string(), "首页" }
    MenuItem { index: "2".to_string(), "用户管理" }
    MenuItem { index: "3".to_string(), "系统设置" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("mode", "模式", "MenuMode", "Vertical"),
                    ApiItem::new("collapse", "是否折叠", "bool", "false"),
                    ApiItem::new("default_active", "默认激活项", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Tabs 标签页 ====================
pub fn tabs_page() -> Element {
    rsx! {
        div {
            h1 { "Tabs 标签页" }
            p { style: "color: var(--el-text-color-secondary);", "分隔内容上有关联但属于不同类别的数据集合。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "标签页".to_string(),
                description: Some("使用 Tabs 和 TabPane 组件".to_string()),
                Tabs {
                    model_value: Some("first".to_string()),
                    TabPane { label: "用户管理".to_string(), name: Some("first".to_string()), "用户管理内容" }
                    TabPane { label: "配置管理".to_string(), name: Some("second".to_string()), "配置管理内容" }
                    TabPane { label: "角色管理".to_string(), name: Some("third".to_string()), "角色管理内容" }
                }
            }
            CodeExample {
                code: r#"Tabs {
    model_value: Some("first".to_string()),
    TabPane {
        label: "用户管理".to_string(),
        name: Some("first".to_string()),
        "用户管理内容"
    }
    TabPane {
        label: "配置管理".to_string(),
        name: Some("second".to_string()),
        "配置管理内容"
    }
    TabPane {
        label: "角色管理".to_string(),
        name: Some("third".to_string()),
        "角色管理内容"
    }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "卡片样式" }
            DemoBlock {
                title: "卡片样式".to_string(),
                description: Some("使用 tab_type 属性设置为卡片样式".to_string()),
                Tabs {
                    tab_type: TabType::Card,
                    TabPane { label: "标签一".to_string(), "内容一" }
                    TabPane { label: "标签二".to_string(), "内容二" }
                }
            }
            CodeExample {
                code: r#"Tabs {
    tab_type: TabType::Card,
    TabPane { label: "标签一".to_string(), "内容一" }
    TabPane { label: "标签二".to_string(), "内容二" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "当前激活标签", "Option<String>", "None"),
                    ApiItem::new("tab_type", "样式类型", "TabType", "Default"),
                    ApiItem::new("tab_position", "位置", "TabPosition", "Top"),
                    ApiItem::new("closable", "是否可关闭", "bool", "false"),
                ],
            }
        }
    }
}

// ==================== Steps 步骤条 ====================
pub fn steps_page() -> Element {
    rsx! {
        div {
            h1 { "Steps 步骤条" }
            p { style: "color: var(--el-text-color-secondary);", "引导用户按照流程完成任务的导航条。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "步骤条".to_string(),
                description: Some("使用 active 属性设置当前步骤".to_string()),
                Steps {
                    active: 1,
                    Step { title: "步骤一".to_string(), description: Some("描述一".to_string()) }
                    Step { title: "步骤二".to_string(), description: Some("描述二".to_string()) }
                    Step { title: "步骤三".to_string(), description: Some("描述三".to_string()) }
                }
            }
            CodeExample {
                code: r#"Steps {
    active: 1,
    Step { title: "步骤一".to_string(), description: Some("描述一".to_string()) }
    Step { title: "步骤二".to_string(), description: Some("描述二".to_string()) }
    Step { title: "步骤三".to_string(), description: Some("描述三".to_string()) }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "竖式步骤条" }
            DemoBlock {
                title: "竖式".to_string(),
                description: Some("使用 direction 属性设置为垂直".to_string()),
                div { style: "max-width: 300px;",
                    Steps {
                        active: 2,
                        direction: StepsDirection::Vertical,
                        Step { title: "第一步".to_string(), "注册" }
                        Step { title: "第二步".to_string(), "验证" }
                        Step { title: "第三步".to_string(), "完成" }
                    }
                }
            }
            CodeExample {
                code: r#"Steps {
    active: 2,
    direction: StepsDirection::Vertical,
    Step { title: "第一步".to_string(), "注册" }
    Step { title: "第二步".to_string(), "验证" }
    Step { title: "第三步".to_string(), "完成" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("active", "当前步骤", "u32", "0"),
                    ApiItem::new("direction", "方向", "StepsDirection", "Horizontal"),
                    ApiItem::new("simple", "简洁模式", "bool", "false"),
                    ApiItem::new("space", "间距", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Pagination 分页 ====================
pub fn pagination_page() -> Element {
    rsx! {
        div {
            h1 { "Pagination 分页" }
            p { style: "color: var(--el-text-color-secondary);", "当数据量过多时，使用分页分解数据。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "分页".to_string(),
                description: Some("使用 total 和 current_page 属性".to_string()),
                Pagination { total: 100, current_page: 3, page_size: 10 }
            }
            CodeExample {
                code: r#"Pagination { total: 100, current_page: 3, page_size: 10 }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("total", "总条数", "u32", "0"),
                    ApiItem::new("current_page", "当前页", "u32", "1"),
                    ApiItem::new("page_size", "每页条数", "u32", "10"),
                    ApiItem::new("pager_count", "页码按钮数", "u32", "7"),
                    ApiItem::new("show_total", "显示总条数", "bool", "false"),
                    ApiItem::new("on_current_change", "页码变更事件", "Option<EventHandler<u32>>", "None"),
                ],
            }
        }
    }
}

// ==================== Breadcrumb 面包屑 ====================
pub fn breadcrumb_page() -> Element {
    rsx! {
        div {
            h1 { "Breadcrumb 面包屑" }
            p { style: "color: var(--el-text-color-secondary);", "显示当前页面的路径，快速返回之前的任意页面。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "面包屑".to_string(),
                description: Some("使用 Breadcrumb 和 BreadcrumbItem 组件".to_string()),
                Breadcrumb {
                    BreadcrumbItem { "首页" }
                    BreadcrumbItem { "活动管理" }
                    BreadcrumbItem { "活动详情" }
                }
            }
            CodeExample {
                code: r#"Breadcrumb {
    BreadcrumbItem { "首页" }
    BreadcrumbItem { "活动管理" }
    BreadcrumbItem { "活动详情" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("separator", "分隔符", "BreadcrumbSeparator", "Slash"),
                ],
            }
        }
    }
}

// ==================== Dropdown 下拉菜单 ====================
pub fn dropdown_page() -> Element {
    rsx! {
        div {
            h1 { "Dropdown 下拉菜单" }
            p { style: "color: var(--el-text-color-secondary);", "将动作或菜单折叠到下拉菜单中。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "下拉菜单".to_string(),
                description: Some("使用 Dropdown 和 DropdownMenu 组件".to_string()),
                Dropdown {
                    DropdownMenu {
                        DropdownItem { "黄金糕" }
                        DropdownItem { "狮子头" }
                        DropdownItem { "螺蛳粉" }
                    }
                    Button { variant: ButtonVariant::Primary, "下拉菜单 ▼" }
                }
            }
            CodeExample {
                code: r#"Dropdown {
    DropdownMenu {
        DropdownItem { "黄金糕" }
        DropdownItem { "狮子头" }
        DropdownItem { "螺蛳粉" }
    }
    Button { variant: ButtonVariant::Primary, "下拉菜单 ▼" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("trigger", "触发方式", "DropdownTrigger", "Hover"),
                    ApiItem::new("placement", "弹出位置", "String", "bottom"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("max_height", "最大高度", "u32", "0"),
                ],
            }
        }
    }
}

// ==================== PageHeader 页头 ====================
pub fn page_header_page() -> Element {
    rsx! {
        div {
            h1 { "PageHeader 页头" }
            p { style: "color: var(--el-text-color-secondary);", "如果页面的路径比较简单，推荐使用页头组件。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "页头".to_string(),
                description: Some("使用 title 属性设置标题".to_string()),
                PageHeader { title: "页面标题".to_string(), content: Some("页面描述".to_string()) }
            }
            CodeExample {
                code: r#"PageHeader {
    title: "页面标题".to_string(),
    content: Some("页面描述".to_string())
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("title", "标题", "String", "required"),
                    ApiItem::new("content", "描述", "Option<String>", "None"),
                    ApiItem::new("icon", "图标", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Backtop 回到顶部 ====================
pub fn backtop_page() -> Element {
    rsx! {
        div {
            h1 { "Backtop 回到顶部" }
            p { style: "color: var(--el-text-color-secondary);", "返回页面顶部的操作按钮。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "回到顶部".to_string(),
                description: Some("使用 visibility_height 属性控制显示".to_string()),
                Backtop { visibility_height: 100, "↑" }
            }
            CodeExample {
                code: r#"Backtop { visibility_height: 100, "↑" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("target", "目标元素", "Option<String>", "None"),
                    ApiItem::new("visibility_height", "显示高度", "u32", "200"),
                    ApiItem::new("right", "右边距", "u32", "40"),
                    ApiItem::new("bottom", "底边距", "u32", "40"),
                ],
            }
        }
    }
}
