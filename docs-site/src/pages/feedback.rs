use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Alert 警告 ====================
pub fn alert_page() -> Element {
    rsx! {
        div {
            h1 { "Alert 警告" }
            p { style: "color: var(--el-text-color-secondary);", "用于页面中展示重要的提示信息。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同类型".to_string(),
                description: Some("使用 alert_type 属性定义警告类型".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Alert { title: "成功提示的文案".to_string(), alert_type: AlertType::Success, show_icon: true }
                    Alert { title: "消息提示的文案".to_string(), alert_type: AlertType::Info, show_icon: true }
                    Alert { title: "警告提示的文案".to_string(), alert_type: AlertType::Warning, show_icon: true }
                    Alert { title: "错误提示的文案".to_string(), alert_type: AlertType::Error, show_icon: true }
                }
            }
            CodeExample {
                code: r#"Alert { title: "成功提示的文案".to_string(), alert_type: AlertType::Success, show_icon: true }
Alert { title: "消息提示的文案".to_string(), alert_type: AlertType::Info, show_icon: true }
Alert { title: "警告提示的文案".to_string(), alert_type: AlertType::Warning, show_icon: true }
Alert { title: "错误提示的文案".to_string(), alert_type: AlertType::Error, show_icon: true }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "带描述" }
            DemoBlock {
                title: "带描述信息".to_string(),
                description: Some("使用 description 属性添加描述".to_string()),
                Alert {
                    title: "带描述的警告".to_string(),
                    alert_type: AlertType::Success,
                    description: Some("这是详细的描述信息，用于补充标题的说明。".to_string()),
                    show_icon: true,
                }
            }
            CodeExample {
                code: r#"Alert {
    title: "带描述的警告".to_string(),
    alert_type: AlertType::Success,
    description: Some("这是详细的描述信息，用于补充标题的说明。".to_string()),
    show_icon: true,
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("title", "标题", "String", "required"),
                    ApiItem::new("description", "描述", "Option<String>", "None"),
                    ApiItem::new("alert_type", "类型", "AlertType", "Info"),
                    ApiItem::new("closable", "是否可关闭", "bool", "false"),
                    ApiItem::new("show_icon", "是否显示图标", "bool", "false"),
                    ApiItem::new("center", "是否居中", "bool", "false"),
                    ApiItem::new("on_close", "关闭事件", "Option<EventHandler<MouseEvent>>", "None"),
                ],
            }
        }
    }
}

// ==================== Dialog 对话框 ====================
pub fn dialog_page() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            h1 { "Dialog 对话框" }
            p { style: "color: var(--el-text-color-secondary);", "在保留当前页面状态下，告知用户并承载相关操作。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "对话框".to_string(),
                description: Some("使用 visible 属性控制显示，on_close 处理关闭".to_string()),
                div {
                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| visible.set(true),
                        "打开对话框"
                    }
                    Dialog {
                        visible: visible(),
                        title: Some("提示".to_string()),
                        on_close: move |_| visible.set(false),
                        span { "这是一段对话框内容" }
                    }
                }
            }
            CodeExample {
                code: r#"let mut visible = use_signal(|| false);

Button {
    variant: ButtonVariant::Primary,
    on_click: move |_| visible.set(true),
    "打开对话框"
}
Dialog {
    visible: visible(),
    title: Some("提示".to_string()),
    on_close: move |_| visible.set(false),
    span { "这是一段对话框内容" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("visible", "是否显示", "bool", "false"),
                    ApiItem::new("title", "标题", "Option<String>", "None"),
                    ApiItem::new("width", "宽度", "String", "50%"),
                    ApiItem::new("modal", "是否模态", "bool", "true"),
                    ApiItem::new("close_on_click_modal", "点击遮罩关闭", "bool", "true"),
                    ApiItem::new("close_on_press_escape", "ESC关闭", "bool", "true"),
                    ApiItem::new("on_close", "关闭事件", "Option<EventHandler<()>>", "None"),
                ],
            }
        }
    }
}

// ==================== Drawer 抽屉 ====================
pub fn drawer_page() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            h1 { "Drawer 抽屉" }
            p { style: "color: var(--el-text-color-secondary);", "有些时候, Dialog 组件并不能满足我们的需求，抽屉组件从屏幕边缘滑出。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "抽屉".to_string(),
                description: Some("使用 visible 属性控制显示，direction 设置方向".to_string()),
                div {
                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| visible.set(true),
                        "打开抽屉"
                    }
                    Drawer {
                        visible: visible(),
                        title: Some("抽屉标题".to_string()),
                        direction: DrawerDirection::Rtl,
                        on_close: move |_| visible.set(false),
                        span { "这是抽屉的内容区域" }
                    }
                }
            }
            CodeExample {
                code: r#"let mut visible = use_signal(|| false);

Button {
    variant: ButtonVariant::Primary,
    on_click: move |_| visible.set(true),
    "打开抽屉"
}
Drawer {
    visible: visible(),
    title: Some("抽屉标题".to_string()),
    direction: DrawerDirection::Rtl,
    on_close: move |_| visible.set(false),
    span { "这是抽屉的内容区域" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("visible", "是否显示", "bool", "false"),
                    ApiItem::new("title", "标题", "Option<String>", "None"),
                    ApiItem::new("direction", "方向", "DrawerDirection", "Rtl"),
                    ApiItem::new("size", "尺寸", "String", "30%"),
                    ApiItem::new("modal", "是否模态", "bool", "true"),
                    ApiItem::new("on_close", "关闭事件", "Option<EventHandler<()>>", "None"),
                ],
            }
        }
    }
}

// ==================== Tooltip 文字提示 ====================
pub fn tooltip_page() -> Element {
    rsx! {
        div {
            h1 { "Tooltip 文字提示" }
            p { style: "color: var(--el-text-color-secondary);", "常用于展示鼠标 hover 时的提示信息。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "文字提示".to_string(),
                description: Some("使用 content 属性设置提示内容".to_string()),
                div {
                    style: "display: flex; gap: 40px; padding: 20px 0;",
                    Tooltip {
                        content: "Top 提示文字".to_string(),
                        placement: TooltipPlacement::Top,
                        Button { variant: ButtonVariant::Default, "上" }
                    }
                    Tooltip {
                        content: "Bottom 提示文字".to_string(),
                        placement: TooltipPlacement::Bottom,
                        Button { variant: ButtonVariant::Default, "下" }
                    }
                    Tooltip {
                        content: "Left 提示文字".to_string(),
                        placement: TooltipPlacement::Left,
                        Button { variant: ButtonVariant::Default, "左" }
                    }
                    Tooltip {
                        content: "Right 提示文字".to_string(),
                        placement: TooltipPlacement::Right,
                        Button { variant: ButtonVariant::Default, "右" }
                    }
                }
            }
            CodeExample {
                code: r#"Tooltip {
    content: "Top 提示文字".to_string(),
    placement: TooltipPlacement::Top,
    Button { variant: ButtonVariant::Default, "上" }
}
Tooltip {
    content: "Bottom 提示文字".to_string(),
    placement: TooltipPlacement::Bottom,
    Button { variant: ButtonVariant::Default, "下" }
}
Tooltip {
    content: "Left 提示文字".to_string(),
    placement: TooltipPlacement::Left,
    Button { variant: ButtonVariant::Default, "左" }
}
Tooltip {
    content: "Right 提示文字".to_string(),
    placement: TooltipPlacement::Right,
    Button { variant: ButtonVariant::Default, "右" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("content", "提示内容", "String", "required"),
                    ApiItem::new("placement", "位置", "TooltipPlacement", "Top"),
                    ApiItem::new("effect", "效果", "String", "dark"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                ],
            }
        }
    }
}

// ==================== Popover 弹出框 ====================
pub fn popover_page() -> Element {
    rsx! {
        div {
            h1 { "Popover 弹出框" }
            p { style: "color: var(--el-text-color-secondary);", "比 Tooltip 提供更丰富的内容和交互。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "弹出框".to_string(),
                description: Some("使用 title 和 content 属性设置内容".to_string()),
                div {
                    style: "padding: 20px 0;",
                    Popover {
                        title: Some("标题".to_string()),
                        content: Some("这是弹出框的内容".to_string()),
                        Button { variant: ButtonVariant::Default, "hover 激活" }
                    }
                }
            }
            CodeExample {
                code: r#"Popover {
    title: Some("标题".to_string()),
    content: Some("这是弹出框的内容".to_string()),
    Button { variant: ButtonVariant::Default, "hover 激活" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("title", "标题", "Option<String>", "None"),
                    ApiItem::new("content", "内容", "Option<String>", "None"),
                    ApiItem::new("placement", "位置", "String", "bottom"),
                    ApiItem::new("trigger", "触发方式", "String", "hover"),
                    ApiItem::new("width", "宽度", "u32", "200"),
                ],
            }
        }
    }
}

// ==================== Popconfirm 气泡确认框 ====================
pub fn popconfirm_page() -> Element {
    rsx! {
        div {
            h1 { "Popconfirm 气泡确认框" }
            p { style: "color: var(--el-text-color-secondary);", "点击元素弹出一个简单的气泡确认框。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "确认框".to_string(),
                description: Some("使用 title 属性设置提示内容，on_confirm 处理确认".to_string()),
                Popconfirm {
                    title: "确定删除吗？".to_string(),
                    Button { variant: ButtonVariant::Default, "删除" }
                }
            }
            CodeExample {
                code: r#"Popconfirm {
    title: "确定删除吗？".to_string(),
    Button { variant: ButtonVariant::Default, "删除" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("title", "提示标题", "String", "Are you sure?"),
                    ApiItem::new("confirm_button_text", "确认按钮文字", "String", "Yes"),
                    ApiItem::new("cancel_button_text", "取消按钮文字", "String", "No"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("on_confirm", "确认事件", "Option<EventHandler<()>>", "None"),
                    ApiItem::new("on_cancel", "取消事件", "Option<EventHandler<()>>", "None"),
                ],
            }
        }
    }
}

// ==================== Message 消息提示 ====================
pub fn message_page() -> Element {
    rsx! {
        div {
            h1 { "Message 消息提示" }
            p { style: "color: var(--el-text-color-secondary);", "常用于主动操作后的反馈提示。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同类型".to_string(),
                description: Some("使用 message_type 属性定义消息类型".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Message { message: "成功消息".to_string(), message_type: MessageType::Success }
                    Message { message: "警告消息".to_string(), message_type: MessageType::Warning }
                    Message { message: "错误消息".to_string(), message_type: MessageType::Error }
                    Message { message: "普通消息".to_string(), message_type: MessageType::Info }
                }
            }
            CodeExample {
                code: r#"Message { message: "成功消息".to_string(), message_type: MessageType::Success }
Message { message: "警告消息".to_string(), message_type: MessageType::Warning }
Message { message: "错误消息".to_string(), message_type: MessageType::Error }
Message { message: "普通消息".to_string(), message_type: MessageType::Info }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("message", "消息内容", "String", "required"),
                    ApiItem::new("message_type", "消息类型", "MessageType", "Info"),
                    ApiItem::new("closable", "是否可关闭", "bool", "false"),
                    ApiItem::new("show_icon", "是否显示图标", "bool", "true"),
                    ApiItem::new("center", "是否居中", "bool", "false"),
                    ApiItem::new("duration", "显示时间(ms)", "u64", "3000"),
                ],
            }
        }
    }
}

// ==================== Notification 通知 ====================
pub fn notification_page() -> Element {
    rsx! {
        div {
            h1 { "Notification 通知" }
            p { style: "color: var(--el-text-color-secondary);", "悬浮出现在页面角落，显示全局的通知提醒消息。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "通知".to_string(),
                description: Some("使用 title 和 message 属性设置内容".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Notification {
                        title: "成功".to_string(),
                        message: "这是一条成功的通知消息".to_string(),
                        notification_type: NotificationType::Success,
                    }
                    Notification {
                        title: "警告".to_string(),
                        message: "这是一条警告的通知消息".to_string(),
                        notification_type: NotificationType::Warning,
                    }
                }
            }
            CodeExample {
                code: r#"Notification {
    title: "成功".to_string(),
    message: "这是一条成功的通知消息".to_string(),
    notification_type: NotificationType::Success,
}
Notification {
    title: "警告".to_string(),
    message: "这是一条警告的通知消息".to_string(),
    notification_type: NotificationType::Warning,
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("title", "标题", "String", "required"),
                    ApiItem::new("message", "消息内容", "String", "required"),
                    ApiItem::new("notification_type", "通知类型", "NotificationType", "Info"),
                    ApiItem::new("position", "位置", "String", "top-right"),
                    ApiItem::new("duration", "显示时间(ms)", "u64", "4500"),
                    ApiItem::new("closable", "是否可关闭", "bool", "true"),
                ],
            }
        }
    }
}

// ==================== Spin 加载 ====================
pub fn spin_page() -> Element {
    rsx! {
        div {
            h1 { "Spin 加载" }
            p { style: "color: var(--el-text-color-secondary);", "当区域正在加载数据时给出加载提示。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "加载中".to_string(),
                description: Some("使用 spinning 属性控制显示".to_string()),
                div {
                    style: "padding: 40px;",
                    Spin { spinning: true, tip: Some("加载中...".to_string()) }
                }
            }
            CodeExample {
                code: r#"Spin {
    spinning: true,
    tip: Some("加载中...".to_string())
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("spinning", "是否旋转", "bool", "true"),
                    ApiItem::new("size", "尺寸", "String", "default"),
                    ApiItem::new("tip", "提示文字", "Option<String>", "None"),
                ],
            }
        }
    }
}
