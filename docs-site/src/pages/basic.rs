use dioxus::prelude::*;
// Explicitly import the Element Plus Link component to disambiguate from dioxus router's Link
use dioxus_element_plug::components::link::Link as ElLink;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Button 按钮 ====================
pub fn button_page() -> Element {
    rsx! {
        div {
            h1 { "Button 按钮" }
            p { style: "color: var(--el-text-color-secondary);", "常用的操作按钮。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "按钮类型".to_string(),
                description: Some("使用 variant 属性定义按钮样式".to_string()),
                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",
                    Button { variant: ButtonVariant::Default, "默认按钮" }
                    Button { variant: ButtonVariant::Primary, "主要按钮" }
                    Button { variant: ButtonVariant::Success, "成功按钮" }
                    Button { variant: ButtonVariant::Warning, "警告按钮" }
                    Button { variant: ButtonVariant::Danger, "危险按钮" }
                }
            }
            CodeExample {
                code: r#"Button { variant: ButtonVariant::Default, "默认按钮" }
Button { variant: ButtonVariant::Primary, "主要按钮" }
Button { variant: ButtonVariant::Success, "成功按钮" }
Button { variant: ButtonVariant::Warning, "警告按钮" }
Button { variant: ButtonVariant::Danger, "危险按钮" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用状态" }
            DemoBlock {
                title: "禁用按钮".to_string(),
                description: Some("使用 disabled 属性禁用按钮".to_string()),
                div {
                    style: "display: flex; gap: 12px;",
                    Button { variant: ButtonVariant::Default, disabled: true, "默认按钮" }
                    Button { variant: ButtonVariant::Primary, disabled: true, "主要按钮" }
                }
            }
            CodeExample {
                code: r#"Button { variant: ButtonVariant::Default, disabled: true, "默认按钮" }
Button { variant: ButtonVariant::Primary, disabled: true, "主要按钮" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同尺寸" }
            DemoBlock {
                title: "按钮尺寸".to_string(),
                description: Some("使用 size 属性定义按钮尺寸".to_string()),
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "大尺寸" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Medium), "默认" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), "小尺寸" }
                    Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Mini), "超小" }
                }
            }
            CodeExample {
                code: r#"Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "大尺寸" }
Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Medium), "默认" }
Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), "小尺寸" }
Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Mini), "超小" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "其他样式" }
            DemoBlock {
                title: "圆角和圆形按钮".to_string(),
                description: Some("使用 round 和 circle 属性".to_string()),
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    Button { variant: ButtonVariant::Primary, round: true, "圆角按钮" }
                    Button { variant: ButtonVariant::Primary, circle: true, "○" }
                    Button { variant: ButtonVariant::Primary, loading: true, "加载中" }
                }
            }
            CodeExample {
                code: r#"Button { variant: ButtonVariant::Primary, round: true, "圆角按钮" }
Button { variant: ButtonVariant::Primary, circle: true, "○" }
Button { variant: ButtonVariant::Primary, loading: true, "加载中" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("variant", "按钮类型", "ButtonVariant", "Default"),
                    ApiItem::new("size", "按钮尺寸", "Option<ButtonSize>", "None"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("loading", "是否加载中", "bool", "false"),
                    ApiItem::new("round", "是否圆角", "bool", "false"),
                    ApiItem::new("circle", "是否圆形", "bool", "false"),
                    ApiItem::new("on_click", "点击事件", "Option<EventHandler<MouseEvent>>", "None"),
                ],
            }
        }
    }
}

// ==================== Input 输入框 ====================
pub fn input_page() -> Element {
    let mut input_value = use_signal(|| "".to_string());

    rsx! {
        div {
            h1 { "Input 输入框" }
            p { style: "color: var(--el-text-color-secondary);", "通过鼠标或键盘输入字符。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "输入框".to_string(),
                description: Some("使用 value 和 on_input 实现双向绑定".to_string()),
                div {
                    style: "max-width: 400px;",
                    Input {
                        value: Some(input_value()),
                        placeholder: Some("请输入内容".to_string()),
                        on_input: move |e: FormEvent| input_value.set(e.value()),
                    }
                    p { style: "margin-top: 12px; color: var(--el-text-color-secondary); font-size: 14px;", "输入值: {input_value}" }
                }
            }
            CodeExample {
                code: r#"let mut input_value = use_signal(|| "".to_string());

Input {
    value: Some(input_value()),
    placeholder: Some("请输入内容".to_string()),
    on_input: move |e: FormEvent| input_value.set(e.value()),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同尺寸" }
            DemoBlock {
                title: "输入框尺寸".to_string(),
                description: Some("使用 size 属性定义输入框尺寸".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                    Input { value: Some("大尺寸".to_string()), size: Some(InputSize::Large) }
                    Input { value: Some("默认".to_string()), size: Some(InputSize::Medium) }
                    Input { value: Some("小尺寸".to_string()), size: Some(InputSize::Small) }
                    Input { value: Some("超小".to_string()), size: Some(InputSize::Mini) }
                }
            }
            CodeExample {
                code: r#"Input { value: Some("大尺寸".to_string()), size: Some(InputSize::Large) }
Input { value: Some("默认".to_string()), size: Some(InputSize::Medium) }
Input { value: Some("小尺寸".to_string()), size: Some(InputSize::Small) }
Input { value: Some("超小".to_string()), size: Some(InputSize::Mini) }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用和只读" }
            DemoBlock {
                title: "禁用/只读".to_string(),
                description: Some("使用 disabled 和 readonly 属性".to_string()),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; max-width: 400px;",
                    Input { value: Some("禁用".to_string()), disabled: true }
                    Input { value: Some("只读".to_string()), readonly: true }
                }
            }
            CodeExample {
                code: r#"Input { value: Some("禁用".to_string()), disabled: true }
Input { value: Some("只读".to_string()), readonly: true }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("value", "绑定值", "Option<String>", "None"),
                    ApiItem::new("placeholder", "占位文本", "Option<String>", "None"),
                    ApiItem::new("input_type", "输入类型", "InputType", "Text"),
                    ApiItem::new("size", "输入框尺寸", "Option<InputSize>", "None"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("readonly", "是否只读", "bool", "false"),
                    ApiItem::new("clearable", "是否可清空", "bool", "false"),
                    ApiItem::new("on_input", "输入事件", "Option<EventHandler<FormEvent>>", "None"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<FormEvent>>", "None"),
                ],
            }
        }
    }
}

// ==================== InputNumber 计数器 ====================
pub fn input_number_page() -> Element {
    let mut num = use_signal(|| 0.0);

    rsx! {
        div {
            h1 { "InputNumber 计数器" }
            p { style: "color: var(--el-text-color-secondary);", "仅允许输入标准的数字，可以定义范围。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "数字输入".to_string(),
                description: Some("使用 model_value 和 on_change 绑定数值".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    InputNumber {
                        model_value: num(),
                        on_change: move |v: f64| num.set(v),
                    }
                    span { style: "color: var(--el-text-color-secondary);", "当前值: {num}" }
                }
            }
            CodeExample {
                code: r#"let mut num = use_signal(|| 0.0);

InputNumber {
    model_value: num(),
    on_change: move |v: f64| num.set(v),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "限制范围" }
            DemoBlock {
                title: "设置 min/max".to_string(),
                description: Some("使用 min 和 max 属性限制范围".to_string()),
                InputNumber { model_value: 5.0, min: 0.0, max: 10.0, step: 1.0 }
            }
            CodeExample {
                code: r#"InputNumber { model_value: 5.0, min: 0.0, max: 10.0, step: 1.0 }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "当前值", "f64", "0"),
                    ApiItem::new("min", "最小值", "f64", "f64::MIN"),
                    ApiItem::new("max", "最大值", "f64", "f64::MAX"),
                    ApiItem::new("step", "步长", "f64", "1"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("controls", "是否显示控制按钮", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<f64>>", "None"),
                ],
            }
        }
    }
}

// ==================== Card 卡片 ====================
pub fn card_page() -> Element {
    rsx! {
        div {
            h1 { "Card 卡片" }
            p { style: "color: var(--el-text-color-secondary);", "将信息聚合在卡片容器中展示。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "基础卡片".to_string(),
                description: Some("包含标题、内容和操作".to_string()),
                Card {
                    header: Some("卡片标题".to_string()),
                    div {
                        p { "这是卡片的内容区域。" }
                        p { "可以放置任何内容。" }
                    }
                }
            }
            CodeExample {
                code: r#"Card {
    header: Some("卡片标题".to_string()),
    div {
        p { "这是卡片的内容区域。" }
        p { "可以放置任何内容。" }
    }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "简单卡片" }
            DemoBlock {
                title: "无标题卡片".to_string(),
                description: Some("只有内容区域的卡片".to_string()),
                Card { "只有内容的卡片" }
            }
            CodeExample {
                code: r#"Card { "只有内容的卡片" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("header", "卡片标题", "Option<String>", "None"),
                    ApiItem::new("shadow", "阴影显示时机", "String", "hover"),
                    ApiItem::new("body_style", "内容区域样式", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Tag 标签 ====================
pub fn tag_page() -> Element {
    rsx! {
        div {
            h1 { "Tag 标签" }
            p { style: "color: var(--el-text-color-secondary);", "用于标记和选择。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同类型".to_string(),
                description: Some("使用 tag_type 属性定义标签类型".to_string()),
                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",
                    Tag { tag_type: TagType::Primary, "Primary" }
                    Tag { tag_type: TagType::Success, "Success" }
                    Tag { tag_type: TagType::Info, "Info" }
                    Tag { tag_type: TagType::Warning, "Warning" }
                    Tag { tag_type: TagType::Danger, "Danger" }
                }
            }
            CodeExample {
                code: r#"Tag { tag_type: TagType::Primary, "Primary" }
Tag { tag_type: TagType::Success, "Success" }
Tag { tag_type: TagType::Info, "Info" }
Tag { tag_type: TagType::Warning, "Warning" }
Tag { tag_type: TagType::Danger, "Danger" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同效果" }
            DemoBlock {
                title: "效果".to_string(),
                description: Some("使用 effect 属性定义标签效果".to_string()),
                div {
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",
                    Tag { tag_type: TagType::Primary, effect: TagEffect::Dark, "Dark" }
                    Tag { tag_type: TagType::Primary, effect: TagEffect::Light, "Light" }
                    Tag { tag_type: TagType::Primary, effect: TagEffect::Plain, "Plain" }
                }
            }
            CodeExample {
                code: r#"Tag { tag_type: TagType::Primary, effect: TagEffect::Dark, "Dark" }
Tag { tag_type: TagType::Primary, effect: TagEffect::Light, "Light" }
Tag { tag_type: TagType::Primary, effect: TagEffect::Plain, "Plain" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同尺寸" }
            DemoBlock {
                title: "尺寸".to_string(),
                description: Some("使用 size 属性定义标签尺寸".to_string()),
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    Tag { tag_type: TagType::Primary, size: TagSize::Large, "Large" }
                    Tag { tag_type: TagType::Primary, size: TagSize::Default, "Default" }
                    Tag { tag_type: TagType::Primary, size: TagSize::Small, "Small" }
                }
            }
            CodeExample {
                code: r#"Tag { tag_type: TagType::Primary, size: TagSize::Large, "Large" }
Tag { tag_type: TagType::Primary, size: TagSize::Default, "Default" }
Tag { tag_type: TagType::Primary, size: TagSize::Small, "Small" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("tag_type", "标签类型", "TagType", "Primary"),
                    ApiItem::new("size", "标签尺寸", "TagSize", "Default"),
                    ApiItem::new("effect", "标签效果", "TagEffect", "Light"),
                    ApiItem::new("closable", "是否可关闭", "bool", "false"),
                    ApiItem::new("round", "是否圆角", "bool", "false"),
                    ApiItem::new("hit", "是否有边框高亮", "bool", "false"),
                    ApiItem::new("on_close", "关闭事件", "Option<EventHandler<MouseEvent>>", "None"),
                ],
            }
        }
    }
}

// ==================== Badge 标记 ====================
pub fn badge_page() -> Element {
    rsx! {
        div {
            h1 { "Badge 标记" }
            p { style: "color: var(--el-text-color-secondary);", "出现在按钮、图标旁的数字或状态标记。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "数值标记".to_string(),
                description: Some("使用 value 属性显示标记值".to_string()),
                div {
                    style: "display: flex; gap: 40px; align-items: center;",
                    Badge { value: Some("1".to_string()), Button { variant: ButtonVariant::Primary, "评论" } }
                    Badge { value: Some("12".to_string()), Button { variant: ButtonVariant::Primary, "回复" } }
                }
            }
            CodeExample {
                code: r#"Badge { value: Some("1".to_string()), Button { variant: ButtonVariant::Primary, "评论" } }
Badge { value: Some("12".to_string()), Button { variant: ButtonVariant::Primary, "回复" } }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "最大值" }
            DemoBlock {
                title: "最大值".to_string(),
                description: Some("使用 max 属性限制最大值".to_string()),
                div {
                    style: "display: flex; gap: 40px; align-items: center;",
                    Badge { value: Some("200".to_string()), max: 99, Button { variant: ButtonVariant::Primary, "评论" } }
                }
            }
            CodeExample {
                code: r#"Badge { value: Some("200".to_string()), max: 99, Button { variant: ButtonVariant::Primary, "评论" } }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "小红点" }
            DemoBlock {
                title: "小红点".to_string(),
                description: Some("使用 is_dot 属性显示小红点".to_string()),
                div {
                    style: "display: flex; gap: 40px; align-items: center;",
                    Badge { is_dot: true, Button { variant: ButtonVariant::Primary, "数据查询" } }
                }
            }
            CodeExample {
                code: r#"Badge { is_dot: true, Button { variant: ButtonVariant::Primary, "数据查询" } }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("value", "显示值", "Option<String>", "None"),
                    ApiItem::new("max", "最大值", "u32", "99"),
                    ApiItem::new("is_dot", "显示小点", "bool", "false"),
                    ApiItem::new("hidden", "是否隐藏", "bool", "false"),
                    ApiItem::new("badge_type", "标记类型", "BadgeType", "Danger"),
                    ApiItem::new("show_zero", "值为零时是否显示", "bool", "true"),
                ],
            }
        }
    }
}

// ==================== Link 文字链接 ====================
pub fn link_page() -> Element {
    rsx! {
        div {
            h1 { "Link 文字链接" }
            p { style: "color: var(--el-text-color-secondary);", "文字超链接。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同类型".to_string(),
                description: Some("使用 link_type 属性定义链接类型".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    ElLink { link_type: LinkType::Default, "Default" }
                    ElLink { link_type: LinkType::Primary, "Primary" }
                    ElLink { link_type: LinkType::Success, "Success" }
                    ElLink { link_type: LinkType::Warning, "Warning" }
                    ElLink { link_type: LinkType::Danger, "Danger" }
                    ElLink { link_type: LinkType::Info, "Info" }
                }
            }
            CodeExample {
                code: r#"ElLink { link_type: LinkType::Default, "Default" }
ElLink { link_type: LinkType::Primary, "Primary" }
ElLink { link_type: LinkType::Success, "Success" }
ElLink { link_type: LinkType::Warning, "Warning" }
ElLink { link_type: LinkType::Danger, "Danger" }
ElLink { link_type: LinkType::Info, "Info" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用状态" }
            DemoBlock {
                title: "禁用链接".to_string(),
                description: Some("使用 disabled 属性禁用链接".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    ElLink { link_type: LinkType::Primary, disabled: true, "禁用链接" }
                }
            }
            CodeExample {
                code: r#"ElLink { link_type: LinkType::Primary, disabled: true, "禁用链接" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("link_type", "链接类型", "LinkType", "Default"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("underline", "下划线", "LinkUnderline", "Hover"),
                    ApiItem::new("href", "链接地址", "Option<String>", "None"),
                ],
            }
        }
    }
}

// ==================== Divider 分割线 ====================
pub fn divider_page() -> Element {
    rsx! {
        div {
            h1 { "Divider 分割线" }
            p { style: "color: var(--el-text-color-secondary);", "区隔内容的分割线。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "水平分割线".to_string(),
                description: Some("默认水平方向的分割线".to_string()),
                div {
                    p { "青春是一个短暂的美梦, 当你醒来时, 它早已消失无踪" }
                    Divider {}
                    p { "少量的邪恶足以抵消全部高贵的品质, 害得人声名狼藉" }
                }
            }
            CodeExample {
                code: r#"p { "青春是一个短暂的美梦, 当你醒来时, 它早已消失无踪" }
Divider {}
p { "少量的邪恶足以抵消全部高贵的品质, 害得人声名狼藉" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "垂直分割线" }
            DemoBlock {
                title: "垂直方向".to_string(),
                description: Some("使用 direction 属性设置为垂直".to_string()),
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    span { "雨纷纷" }
                    Divider { direction: DividerDirection::Vertical }
                    span { "旧故里" }
                    Divider { direction: DividerDirection::Vertical }
                    span { "草木深" }
                }
            }
            CodeExample {
                code: r#"span { "雨纷纷" }
Divider { direction: DividerDirection::Vertical }
span { "旧故里" }
Divider { direction: DividerDirection::Vertical }
span { "草木深" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("direction", "方向", "DividerDirection", "Horizontal"),
                    ApiItem::new("content_position", "内容位置", "DividerContentPosition", "Center"),
                    ApiItem::new("border_style", "边框样式", "String", "solid"),
                ],
            }
        }
    }
}

// ==================== Empty 空状态 ====================
pub fn empty_page() -> Element {
    rsx! {
        div {
            h1 { "Empty 空状态" }
            p { style: "color: var(--el-text-color-secondary);", "空状态时的占位提示。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "空状态".to_string(),
                description: Some("默认空状态组件".to_string()),
                Empty { description: "暂无数据".to_string() }
            }
            CodeExample {
                code: r#"Empty { description: "暂无数据".to_string() }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("image", "图片地址", "Option<String>", "None"),
                    ApiItem::new("image_size", "图片大小", "Option<u32>", "None"),
                    ApiItem::new("description", "描述文本", "String", "No Data"),
                ],
            }
        }
    }
}

// ==================== Avatar 头像 ====================
pub fn avatar_page() -> Element {
    rsx! {
        div {
            h1 { "Avatar 头像" }
            p { style: "color: var(--el-text-color-secondary);", "用图标、图片或者字符占位的展示组件。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "不同尺寸".to_string(),
                description: Some("使用 size 属性定义头像尺寸".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    Avatar { size: AvatarSize::Large, "U" }
                    Avatar { size: AvatarSize::Default, "U" }
                    Avatar { size: AvatarSize::Small, "U" }
                }
            }
            CodeExample {
                code: r#"Avatar { size: AvatarSize::Large, "U" }
Avatar { size: AvatarSize::Default, "U" }
Avatar { size: AvatarSize::Small, "U" }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同形状" }
            DemoBlock {
                title: "形状".to_string(),
                description: Some("使用 shape 属性定义头像形状".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    Avatar { shape: AvatarShape::Circle, "C" }
                    Avatar { shape: AvatarShape::Square, "S" }
                }
            }
            CodeExample {
                code: r#"Avatar { shape: AvatarShape::Circle, "C" }
Avatar { shape: AvatarShape::Square, "S" }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("src", "图片地址", "Option<String>", "None"),
                    ApiItem::new("icon", "图标类名", "Option<String>", "None"),
                    ApiItem::new("size", "头像尺寸", "AvatarSize", "Default"),
                    ApiItem::new("shape", "头像形状", "AvatarShape", "Circle"),
                ],
            }
        }
    }
}

// ==================== ButtonGroup 按钮组 ====================
pub fn button_group_page() -> Element {
    rsx! {
        div {
            h1 { "ButtonGroup 按钮组" }
            p { style: "color: var(--el-text-color-secondary);", "以按钮组的方式出现，常用于多项类似操作。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "按钮组".to_string(),
                description: Some("使用 ButtonGroup 包裹多个 Button".to_string()),
                ButtonGroup {
                    Button { variant: ButtonVariant::Primary, "上一页" }
                    Button { variant: ButtonVariant::Primary, "下一页" }
                }
            }
            CodeExample {
                code: r#"ButtonGroup {
    Button { variant: ButtonVariant::Primary, "上一页" }
    Button { variant: ButtonVariant::Primary, "下一页" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("size", "按钮组尺寸", "ButtonGroupSize", "Default"),
                    ApiItem::new("direction", "方向", "ButtonGroupDirection", "Horizontal"),
                ],
            }
        }
    }
}

// ==================== Icon 图标 ====================
pub fn icon_page() -> Element {
    rsx! {
        div {
            h1 { "Icon 图标" }
            p { style: "color: var(--el-text-color-secondary);", "Element Plus 提供的常用图标集合。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "图标使用".to_string(),
                description: Some("通过 icons feature 使用 SVG 图标组件".to_string()),
                div {
                    style: "display: flex; gap: 20px; font-size: 24px;",
                    span { "📦" }
                    span { "🔍" }
                    span { "⚙️" }
                    span { "❤️" }
                    span { "⭐" }
                }
            }
            CodeExample {
                code: r#"// 在 Cargo.toml 中启用 icons feature
// [dependencies]
// dioxus-element-plug = { features = ["icons"] }

// 使用 Element Plus 图标
use element_icons::element::{Search, Setting};

rsx! {
    Button { Search {}, "搜索" }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("name", "图标名称", "String", "required"),
                    ApiItem::new("size", "图标大小", "Option<String>", "None"),
                    ApiItem::new("color", "图标颜色", "Option<String>", "None"),
                ],
            }
        }
    }
}
