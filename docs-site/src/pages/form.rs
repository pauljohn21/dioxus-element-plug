use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::components::demo_block::DemoBlock;
use crate::components::code_example::CodeExample;
use crate::components::api_table::{ApiTable, ApiItem};

// ==================== Select 选择器 ====================
pub fn select_page() -> Element {
    let mut selected = use_signal(|| None);

    rsx! {
        div {
            h1 { "Select 选择器" }
            p { style: "color: var(--el-text-color-secondary);", "当选项过多时，使用下拉菜单展示并选择内容。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "基础选择器".to_string(),
                description: Some("使用 options 和 on_change 绑定选中值".to_string()),
                div {
                    style: "max-width: 300px;",
                    Select {
                        model_value: selected(),
                        options: vec![
                            SelectOption::new("1", "选项一"),
                            SelectOption::new("2", "选项二"),
                            SelectOption::new("3", "选项三"),
                        ],
                        placeholder: "请选择".to_string(),
                        on_change: move |v: String| selected.set(Some(v)),
                    }
                }
            }
            CodeExample {
                code: r#"let mut selected = use_signal(|| None);

Select {
    model_value: selected(),
    options: vec![
        SelectOption::new("1", "选项一"),
        SelectOption::new("2", "选项二"),
        SelectOption::new("3", "选项三"),
    ],
    placeholder: "请选择".to_string(),
    on_change: move |v: String| selected.set(Some(v)),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用状态" }
            DemoBlock {
                title: "禁用选择器".to_string(),
                description: Some("使用 disabled 属性禁用选择器".to_string()),
                div {
                    style: "max-width: 300px;",
                    Select {
                        disabled: true,
                        options: vec![SelectOption::new("1", "选项一")],
                        placeholder: "请选择".to_string(),
                    }
                }
            }
            CodeExample {
                code: r#"Select {
    disabled: true,
    options: vec![SelectOption::new("1", "选项一")],
    placeholder: "请选择".to_string(),
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "选中值", "Option<String>", "None"),
                    ApiItem::new("options", "选项列表", "Vec<SelectOption>", "[]"),
                    ApiItem::new("placeholder", "占位文本", "String", "Select"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("clearable", "是否可清空", "bool", "false"),
                    ApiItem::new("filterable", "是否可搜索", "bool", "false"),
                    ApiItem::new("multiple", "是否多选", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<String>>", "None"),
                ],
            }
        }
    }
}

// ==================== Switch 开关 ====================
pub fn switch_page() -> Element {
    let mut switch_on = use_signal(|| false);

    rsx! {
        div {
            h1 { "Switch 开关" }
            p { style: "color: var(--el-text-color-secondary);", "表示两种相互对立的状态间的切换。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "开关".to_string(),
                description: Some("使用 model_value 和 on_change 绑定状态".to_string()),
                div {
                    style: "display: flex; gap: 20px; align-items: center;",
                    Switch {
                        model_value: switch_on(),
                        on_change: move |v: bool| switch_on.set(v),
                    }
                    span { "当前状态: {switch_on}" }
                }
            }
            CodeExample {
                code: r#"let mut switch_on = use_signal(|| false);

Switch {
    model_value: switch_on(),
    on_change: move |v: bool| switch_on.set(v),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "不同尺寸" }
            DemoBlock {
                title: "尺寸".to_string(),
                description: Some("使用 size 属性定义开关尺寸".to_string()),
                div {
                    style: "display: flex; gap: 20px; align-items: center;",
                    Switch { model_value: true, size: SwitchSize::Large }
                    Switch { model_value: true, size: SwitchSize::Default }
                    Switch { model_value: true, size: SwitchSize::Small }
                }
            }
            CodeExample {
                code: r#"Switch { model_value: true, size: SwitchSize::Large }
Switch { model_value: true, size: SwitchSize::Default }
Switch { model_value: true, size: SwitchSize::Small }"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "文字描述" }
            DemoBlock {
                title: "文字描述".to_string(),
                description: Some("使用 active_text 和 inactive_text 设置文字".to_string()),
                Switch {
                    model_value: true,
                    active_text: Some("开".to_string()),
                    inactive_text: Some("关".to_string()),
                }
            }
            CodeExample {
                code: r#"Switch {
    model_value: true,
    active_text: Some("开".to_string()),
    inactive_text: Some("关".to_string()),
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "当前状态", "bool", "false"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("loading", "是否加载中", "bool", "false"),
                    ApiItem::new("size", "开关尺寸", "SwitchSize", "Default"),
                    ApiItem::new("active_text", "激活时文字", "Option<String>", "None"),
                    ApiItem::new("inactive_text", "未激活时文字", "Option<String>", "None"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<bool>>", "None"),
                ],
            }
        }
    }
}

// ==================== Checkbox 多选框 ====================
pub fn checkbox_page() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div {
            h1 { "Checkbox 多选框" }
            p { style: "color: var(--el-text-color-secondary);", "一组备选项中进行多选。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "多选框".to_string(),
                description: Some("使用 model_value 和 on_change 绑定状态".to_string()),
                div {
                    style: "display: flex; gap: 16px; align-items: center;",
                    Checkbox {
                        model_value: checked(),
                        on_change: move |v: bool| checked.set(v),
                        label: Some("复选框 A".to_string()),
                    }
                    span { "状态: {checked}" }
                }
            }
            CodeExample {
                code: r#"let mut checked = use_signal(|| false);

Checkbox {
    model_value: checked(),
    on_change: move |v: bool| checked.set(v),
    label: Some("复选框 A".to_string()),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用状态" }
            DemoBlock {
                title: "禁用".to_string(),
                description: Some("使用 disabled 属性禁用".to_string()),
                div {
                    style: "display: flex; gap: 16px;",
                    Checkbox { model_value: false, disabled: true, label: Some("禁用".to_string()) }
                    Checkbox { model_value: true, disabled: true, label: Some("选中且禁用".to_string()) }
                }
            }
            CodeExample {
                code: r#"Checkbox { model_value: false, disabled: true, label: Some("禁用".to_string()) }
Checkbox { model_value: true, disabled: true, label: Some("选中且禁用".to_string()) }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "是否选中", "bool", "false"),
                    ApiItem::new("label", "标签文本", "Option<String>", "None"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("indeterminate", "不确定状态", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<bool>>", "None"),
                ],
            }
        }
    }
}

// ==================== Radio 单选框 ====================
pub fn radio_page() -> Element {
    rsx! {
        div {
            h1 { "Radio 单选框" }
            p { style: "color: var(--el-text-color-secondary);", "在一组备选项中进行单选。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "单选框组".to_string(),
                description: Some("使用 RadioGroup 包裹多个 Radio".to_string()),
                RadioGroup {
                    Radio { value: "1".to_string(), model_value: true, label: Some("选项一".to_string()) }
                    Radio { value: "2".to_string(), model_value: false, label: Some("选项二".to_string()) }
                    Radio { value: "3".to_string(), model_value: false, label: Some("选项三".to_string()) }
                }
            }
            CodeExample {
                code: r#"RadioGroup {
    Radio { value: "1".to_string(), model_value: true, label: Some("选项一".to_string()) }
    Radio { value: "2".to_string(), model_value: false, label: Some("选项二".to_string()) }
    Radio { value: "3".to_string(), model_value: false, label: Some("选项三".to_string()) }
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "按钮样式" }
            DemoBlock {
                title: "单选按钮".to_string(),
                description: Some("使用 RadioButton 组件".to_string()),
                RadioGroup {
                    RadioButton { value: "1".to_string(), model_value: true, label: Some("北京".to_string()) }
                    RadioButton { value: "2".to_string(), model_value: false, label: Some("上海".to_string()) }
                    RadioButton { value: "3".to_string(), model_value: false, label: Some("广州".to_string()) }
                }
            }
            CodeExample {
                code: r#"RadioGroup {
    RadioButton { value: "1".to_string(), model_value: true, label: Some("北京".to_string()) }
    RadioButton { value: "2".to_string(), model_value: false, label: Some("上海".to_string()) }
    RadioButton { value: "3".to_string(), model_value: false, label: Some("广州".to_string()) }
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("value", "单选框值", "String", "required"),
                    ApiItem::new("model_value", "是否选中", "bool", "false"),
                    ApiItem::new("label", "标签文本", "Option<String>", "None"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<bool>>", "None"),
                ],
            }
        }
    }
}

// ==================== Slider 滑块 ====================
pub fn slider_page() -> Element {
    let mut value = use_signal(|| 30.0);

    rsx! {
        div {
            h1 { "Slider 滑块" }
            p { style: "color: var(--el-text-color-secondary);", "通过拖动滑块在一个固定区间内进行选择。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "滑块".to_string(),
                description: Some("使用 model_value 和 on_change 绑定值".to_string()),
                div {
                    style: "max-width: 400px;",
                    Slider {
                        model_value: value(),
                        on_change: move |v: f64| value.set(v),
                    }
                    p { style: "margin-top: 12px; color: var(--el-text-color-secondary);", "当前值: {value}" }
                }
            }
            CodeExample {
                code: r#"let mut value = use_signal(|| 30.0);

Slider {
    model_value: value(),
    on_change: move |v: f64| value.set(v),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "禁用状态" }
            DemoBlock {
                title: "禁用滑块".to_string(),
                description: Some("使用 disabled 属性禁用滑块".to_string()),
                div {
                    style: "max-width: 400px;",
                    Slider { model_value: 50.0, disabled: true }
                }
            }
            CodeExample {
                code: r#"Slider { model_value: 50.0, disabled: true }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "当前值", "f64", "0"),
                    ApiItem::new("min", "最小值", "f64", "0"),
                    ApiItem::new("max", "最大值", "f64", "100"),
                    ApiItem::new("step", "步长", "f64", "1"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("show_input", "是否显示输入框", "bool", "false"),
                    ApiItem::new("show_stops", "是否显示间断点", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<f64>>", "None"),
                ],
            }
        }
    }
}

// ==================== Rate 评分 ====================
pub fn rate_page() -> Element {
    let mut rating = use_signal(|| 3.0);

    rsx! {
        div {
            h1 { "Rate 评分" }
            p { style: "color: var(--el-text-color-secondary);", "评分组件。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "评分".to_string(),
                description: Some("使用 model_value 和 on_change 绑定评分".to_string()),
                div {
                    Rate {
                        model_value: rating(),
                        on_change: move |v: f64| rating.set(v),
                    }
                    span { style: "margin-left: 12px; color: var(--el-text-color-secondary);", "评分: {rating}" }
                }
            }
            CodeExample {
                code: r#"let mut rating = use_signal(|| 3.0);

Rate {
    model_value: rating(),
    on_change: move |v: f64| rating.set(v),
}"#.to_string(),
            }

            h2 { style: "margin-top: 32px;", "半星" }
            DemoBlock {
                title: "允许半选".to_string(),
                description: Some("使用 allow_half 属性允许半星".to_string()),
                Rate { model_value: 3.5, allow_half: true }
            }
            CodeExample {
                code: r#"Rate { model_value: 3.5, allow_half: true }"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("model_value", "当前评分", "f64", "0"),
                    ApiItem::new("max", "最大分值", "u32", "5"),
                    ApiItem::new("disabled", "是否禁用", "bool", "false"),
                    ApiItem::new("allow_half", "是否允许半选", "bool", "false"),
                    ApiItem::new("clearable", "是否可清空", "bool", "false"),
                    ApiItem::new("show_text", "是否显示文字", "bool", "false"),
                    ApiItem::new("show_score", "是否显示分数", "bool", "false"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<f64>>", "None"),
                ],
            }
        }
    }
}

// ==================== Transfer 穿梭框 ====================
pub fn transfer_page() -> Element {
    rsx! {
        div {
            h1 { "Transfer 穿梭框" }
            p { style: "color: var(--el-text-color-secondary);", "用于双栏列表选择元素。" }

            h2 { style: "margin-top: 32px;", "基础用法" }
            DemoBlock {
                title: "穿梭框".to_string(),
                description: Some("使用 data 属性传入数据列表".to_string()),
                Transfer {
                    data: vec![
                        TransferItem::new("1", "选项一"),
                        TransferItem::new("2", "选项二"),
                        TransferItem::new("3", "选项三"),
                        TransferItem::new("4", "选项四"),
                    ],
                }
            }
            CodeExample {
                code: r#"Transfer {
    data: vec![
        TransferItem::new("1", "选项一"),
        TransferItem::new("2", "选项二"),
        TransferItem::new("3", "选项三"),
        TransferItem::new("4", "选项四"),
    ],
}"#.to_string(),
            }

            ApiTable {
                title: "Attributes".to_string(),
                headers: vec!["参数".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()],
                items: vec![
                    ApiItem::new("data", "数据列表", "Vec<TransferItem>", "[]"),
                    ApiItem::new("on_change", "变更事件", "Option<EventHandler<Vec<String>>>", "None"),
                ],
            }
        }
    }
}
