use dioxus::prelude::*;
// Explicitly import Link to disambiguate from dioxus_element_plug's Link component
use dioxus::router::components::Link;
use dioxus_element_plug::prelude::*;

use crate::Route;

/// Layout component that wraps all routes.
///
/// All routes inside `#[layout(DocLayout)]` will be rendered in the `Outlet` below.
/// This ensures `Header` and `Sidebar` are within the `Router` context, so they
/// can use `Link` for navigation.
#[component]
pub fn DocLayout() -> Element {
    // Retrieve the dark mode signal from context (provided by App)
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: "docs-app",
            style: "min-height: 100vh; display: flex; flex-direction: column; background: var(--el-bg-color);",

            Header {
                is_dark: is_dark(),
                on_theme_toggle: move |_| is_dark.set(!is_dark()),
            }

            div {
                style: "display: flex; flex: 1;",

                Sidebar {}

                main {
                    class: "docs-main",
                    style: "flex: 1; padding: 40px; max-width: 900px; margin: 0 auto; width: 100%; margin-left: 260px;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub is_dark: bool,
    pub on_theme_toggle: EventHandler<()>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let theme_icon = if props.is_dark { "☀️" } else { "🌙" };

    rsx! {
        header {
            style: "height: 60px; display: flex; align-items: center; justify-content: space-between; padding: 0 40px; border-bottom: 1px solid var(--el-border-color); background: var(--el-bg-color); position: sticky; top: 0; z-index: 100;",

            div {
                style: "display: flex; align-items: center; gap: 40px;",

                Link {
                    to: Route::Index {},
                    style: "text-decoration: none;",
                    span {
                        style: "font-size: 20px; font-weight: 600; color: var(--el-color-primary);",
                        "Dioxus Element Plug"
                    }
                }

                nav {
                    style: "display: flex; gap: 24px;",
                    Link {
                        to: Route::Installation {},
                        style: "color: var(--el-text-color-regular); text-decoration: none; font-size: 14px;",
                        "指南"
                    }
                    Link {
                        to: Route::ComponentPage { name: "button".to_string() },
                        style: "color: var(--el-text-color-regular); text-decoration: none; font-size: 14px;",
                        "组件"
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                span {
                    style: "font-size: 14px; color: var(--el-text-color-secondary);",
                    "v0.3.0"
                }

                Button {
                    variant: ButtonVariant::Default,
                    on_click: move |_| props.on_theme_toggle.call(()),
                    "{theme_icon}"
                }
            }
        }
    }
}

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside {
            style: "width: 260px; height: calc(100vh - 60px); overflow-y: auto; padding: 24px 0; border-right: 1px solid var(--el-border-color); background: var(--el-bg-color); position: fixed; top: 60px; left: 0;",

            // 开发指南
            NavGroup { title: "开发指南",
                NavItem { to: Route::Index {}, label: "介绍" }
                NavItem { to: Route::Installation {}, label: "安装" }
                NavItem { to: Route::QuickStart {}, label: "快速开始" }
            }

            NavGroup { title: "基础组件",
                NavItem { to: Route::ComponentPage { name: "button".to_string() }, label: "Button 按钮" }
                NavItem { to: Route::ComponentPage { name: "input".to_string() }, label: "Input 输入框" }
                NavItem { to: Route::ComponentPage { name: "input-number".to_string() }, label: "InputNumber 计数器" }
                NavItem { to: Route::ComponentPage { name: "card".to_string() }, label: "Card 卡片" }
                NavItem { to: Route::ComponentPage { name: "tag".to_string() }, label: "Tag 标签" }
                NavItem { to: Route::ComponentPage { name: "badge".to_string() }, label: "Badge 标记" }
                NavItem { to: Route::ComponentPage { name: "link".to_string() }, label: "Link 文字链接" }
                NavItem { to: Route::ComponentPage { name: "divider".to_string() }, label: "Divider 分割线" }
                NavItem { to: Route::ComponentPage { name: "empty".to_string() }, label: "Empty 空状态" }
                NavItem { to: Route::ComponentPage { name: "avatar".to_string() }, label: "Avatar 头像" }
                NavItem { to: Route::ComponentPage { name: "button-group".to_string() }, label: "ButtonGroup 按钮组" }
                NavItem { to: Route::ComponentPage { name: "icon".to_string() }, label: "Icon 图标" }
            }

            NavGroup { title: "表单组件",
                NavItem { to: Route::ComponentPage { name: "select".to_string() }, label: "Select 选择器" }
                NavItem { to: Route::ComponentPage { name: "switch".to_string() }, label: "Switch 开关" }
                NavItem { to: Route::ComponentPage { name: "checkbox".to_string() }, label: "Checkbox 多选框" }
                NavItem { to: Route::ComponentPage { name: "radio".to_string() }, label: "Radio 单选框" }
                NavItem { to: Route::ComponentPage { name: "slider".to_string() }, label: "Slider 滑块" }
                NavItem { to: Route::ComponentPage { name: "rate".to_string() }, label: "Rate 评分" }
                NavItem { to: Route::ComponentPage { name: "transfer".to_string() }, label: "Transfer 穿梭框" }
            }

            NavGroup { title: "数据展示",
                NavItem { to: Route::ComponentPage { name: "table".to_string() }, label: "Table 表格" }
                NavItem { to: Route::ComponentPage { name: "progress".to_string() }, label: "Progress 进度条" }
                NavItem { to: Route::ComponentPage { name: "tree".to_string() }, label: "Tree 树形控件" }
                NavItem { to: Route::ComponentPage { name: "image".to_string() }, label: "Image 图片" }
                NavItem { to: Route::ComponentPage { name: "collapse".to_string() }, label: "Collapse 折叠面板" }
                NavItem { to: Route::ComponentPage { name: "result".to_string() }, label: "Result 结果" }
                NavItem { to: Route::ComponentPage { name: "skeleton".to_string() }, label: "Skeleton 骨架屏" }
                NavItem { to: Route::ComponentPage { name: "space".to_string() }, label: "Space 间距" }
            }

            NavGroup { title: "导航组件",
                NavItem { to: Route::ComponentPage { name: "menu".to_string() }, label: "Menu 导航菜单" }
                NavItem { to: Route::ComponentPage { name: "tabs".to_string() }, label: "Tabs 标签页" }
                NavItem { to: Route::ComponentPage { name: "steps".to_string() }, label: "Steps 步骤条" }
                NavItem { to: Route::ComponentPage { name: "pagination".to_string() }, label: "Pagination 分页" }
                NavItem { to: Route::ComponentPage { name: "breadcrumb".to_string() }, label: "Breadcrumb 面包屑" }
                NavItem { to: Route::ComponentPage { name: "dropdown".to_string() }, label: "Dropdown 下拉菜单" }
                NavItem { to: Route::ComponentPage { name: "page-header".to_string() }, label: "PageHeader 页头" }
                NavItem { to: Route::ComponentPage { name: "backtop".to_string() }, label: "Backtop 回到顶部" }
            }

            NavGroup { title: "反馈组件",
                NavItem { to: Route::ComponentPage { name: "alert".to_string() }, label: "Alert 警告" }
                NavItem { to: Route::ComponentPage { name: "dialog".to_string() }, label: "Dialog 对话框" }
                NavItem { to: Route::ComponentPage { name: "drawer".to_string() }, label: "Drawer 抽屉" }
                NavItem { to: Route::ComponentPage { name: "tooltip".to_string() }, label: "Tooltip 文字提示" }
                NavItem { to: Route::ComponentPage { name: "popover".to_string() }, label: "Popover 弹出框" }
                NavItem { to: Route::ComponentPage { name: "popconfirm".to_string() }, label: "Popconfirm 气泡确认框" }
                NavItem { to: Route::ComponentPage { name: "message".to_string() }, label: "Message 消息提示" }
                NavItem { to: Route::ComponentPage { name: "notification".to_string() }, label: "Notification 通知" }
                NavItem { to: Route::ComponentPage { name: "spin".to_string() }, label: "Spin 加载" }
            }

            NavGroup { title: "布局组件",
                NavItem { to: Route::ComponentPage { name: "container".to_string() }, label: "Container 布局容器" }
                NavItem { to: Route::ComponentPage { name: "row-col".to_string() }, label: "Row/Col 栅格布局" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavGroupProps {
    pub title: String,
    pub children: Element,
}

#[component]
fn NavGroup(props: NavGroupProps) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 24px;",
            h3 {
                style: "padding: 0 20px; margin: 0 0 8px 0; font-size: 12px; color: var(--el-text-color-secondary); font-weight: 600;",
                "{props.title}"
            }
            div {
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    pub to: Route,
    pub label: String,
}

#[component]
fn NavItem(props: NavItemProps) -> Element {
    let current_route = use_route::<Route>();
    let is_active = current_route == props.to;

    let style = if is_active {
        "display: block; padding: 8px 20px; color: var(--el-color-primary); background: var(--el-color-primary-light-9); font-size: 14px; text-decoration: none; font-weight: 500;"
    } else {
        "display: block; padding: 8px 20px; color: var(--el-text-color-regular); text-decoration: none; font-size: 14px; transition: all 0.2s;"
    };

    rsx! {
        Link {
            to: props.to,
            style: "{style}",
            "{props.label}"
        }
    }
}
