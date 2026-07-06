//! Component Showcase - 验证 dioxus-element-plug v0.2.0 的主要组件功能
//!
//! 覆盖组件：Button, Input, Select, Switch, Alert, Tag, Card,
//! Dialog, Table, Tree, Cascader, Transfer, Progress

use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use dioxus_element_plug::components::table::{TableColumn, SortOrder};
use dioxus_element_plug::components::tree::TreeNodeData;
use dioxus_element_plug::components::cascader::CascaderOption;
use dioxus_element_plug::components::transfer::TransferItem;
use dioxus_element_plug::components::progress::{ProgressStatus};
use std::collections::HashMap;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "//unpkg.com/element-plus@2.4.4/dist/index.css"
        }

        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;",

            h1 { style: "font-size: 28px; margin-bottom: 24px; color: #303133;", "Dioxus Element Plus 组件展示" }

            ButtonSection {}
            InputSection {}
            SelectSection {}
            SwitchSection {}
            AlertSection {}
            TagSection {}
            CardSection {}
            DialogSection {}
            TableSection {}
            TreeSection {}
            CascaderSection {}
            TransferSection {}
            ProgressSection {}
        }
    }
}

// ============================================================
// 1. Button 组件
// ============================================================
#[component]
fn ButtonSection() -> Element {
    let mut click_count = use_signal(|| 0i32);
    let click_count_val = *click_count.read();

    rsx! {
        Card {
            header: Some("Button 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-bottom: 16px;",

                Button {
                    variant: ButtonVariant::Default,
                    on_click: move |_: MouseEvent| click_count += 1,
                    "Default (点击: {click_count_val})"
                }
                Button { variant: ButtonVariant::Primary, "Primary" }
                Button { variant: ButtonVariant::Success, "Success" }
                Button { variant: ButtonVariant::Warning, "Warning" }
                Button { variant: ButtonVariant::Danger, "Danger" }
                Button { variant: ButtonVariant::Info, "Info" }
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-bottom: 16px;",

                Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "Large" }
                Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Medium), "Medium" }
                Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), "Small" }
                Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Mini), "Mini" }
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",

                Button { variant: ButtonVariant::Primary, disabled: true, "Disabled" }
                Button { variant: ButtonVariant::Primary, round: true, "Round" }
            }
        }
    }
}

// ============================================================
// 2. Input 组件
// ============================================================
#[component]
fn InputSection() -> Element {
    let mut text = use_signal(|| String::new());
    let text_val = text();

    rsx! {
        Card {
            header: Some("Input 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; flex-direction: column; gap: 12px; max-width: 400px;",

                Input {
                    value: text_val.clone(),
                    placeholder: Some("请输入文本".to_string()),
                    on_change: move |e: Event<FormData>| {
                        text.set(e.value());
                    },
                }

                div { style: "color: #606266; font-size: 14px;", "当前值: {text_val}" }

                Input {
                    placeholder: Some("密码输入".to_string()),
                    input_type: InputType::Password,
                }

                Input {
                    placeholder: Some("带清除按钮".to_string()),
                    clearable: true,
                }
            }
        }
    }
}

// ============================================================
// 3. Select 组件
// ============================================================
#[component]
fn SelectSection() -> Element {
    let mut selected = use_signal(|| None::<String>);
    let selected_val = selected();
    let selected_display: String = selected_val.as_ref().cloned().unwrap_or_else(|| "未选择".to_string());

    rsx! {
        Card {
            header: Some("Select 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "max-width: 400px;",

                Select {
                    model_value: selected_val,
                    options: vec![
                        SelectOption::new("1", "北京"),
                        SelectOption::new("2", "上海"),
                        SelectOption::new("3", "广州"),
                        SelectOption::new("4", "深圳").disabled(true),
                    ],
                    placeholder: "请选择城市".to_string(),
                    on_change: move |val: String| {
                        selected.set(Some(val));
                    },
                }

                div {
                    style: "color: #606266; font-size: 14px; margin-top: 8px;",
                    "选中: {selected_display}"
                }
            }
        }
    }
}

// ============================================================
// 4. Switch 组件
// ============================================================
#[component]
fn SwitchSection() -> Element {
    let mut switch1 = use_signal(|| false);
    let mut switch2 = use_signal(|| true);
    let s1 = *switch1.read();
    let s2 = *switch2.read();

    rsx! {
        Card {
            header: Some("Switch 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; gap: 24px; align-items: center;",

                div {
                    style: "display: flex; gap: 8px; align-items: center;",
                    Switch {
                        model_value: s1,
                        on_change: move |v: bool| switch1.set(v),
                    }
                    span { style: "font-size: 14px; color: #606266;", "状态: {s1}" }
                }

                div {
                    style: "display: flex; gap: 8px; align-items: center;",
                    Switch {
                        model_value: s2,
                        on_change: move |v: bool| switch2.set(v),
                    }
                    span { style: "font-size: 14px; color: #606266;", "状态: {s2}" }
                }

                Switch { model_value: false, disabled: true }
                span { style: "font-size: 14px; color: #909399;", "禁用" }
            }
        }
    }
}

// ============================================================
// 5. Alert 组件
// ============================================================
#[component]
fn AlertSection() -> Element {
    rsx! {
        Card {
            header: Some("Alert 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                Alert {
                    title: "成功提示".to_string(),
                    alert_type: AlertType::Success,
                    show_icon: true,
                    closable: true,
                }
                Alert {
                    title: "警告提示".to_string(),
                    description: Some("这是一条警告描述信息".to_string()),
                    alert_type: AlertType::Warning,
                    show_icon: true,
                    closable: true,
                }
                Alert {
                    title: "错误提示".to_string(),
                    alert_type: AlertType::Error,
                    show_icon: true,
                    closable: false,
                }
                Alert {
                    title: "信息提示".to_string(),
                    alert_type: AlertType::Info,
                    show_icon: true,
                    center: true,
                }
            }
        }
    }
}

// ============================================================
// 6. Tag 组件
// ============================================================
#[component]
fn TagSection() -> Element {
    rsx! {
        Card {
            header: Some("Tag 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px;",

                Tag { tag_type: TagType::Primary, "Primary" }
                Tag { tag_type: TagType::Success, "Success" }
                Tag { tag_type: TagType::Warning, "Warning" }
                Tag { tag_type: TagType::Danger,  "Danger" }
                Tag { tag_type: TagType::Info,    "Info" }

                div { style: "width: 100%; height: 8px;" }

                Tag { tag_type: TagType::Primary, effect: TagEffect::Dark,  "Dark" }
                Tag { tag_type: TagType::Success, effect: TagEffect::Light, "Light" }
                Tag { tag_type: TagType::Warning, effect: TagEffect::Plain,  "Plain" }

                div { style: "width: 100%; height: 8px;" }

                Tag { tag_type: TagType::Primary, size: TagSize::Large,   "Large" }
                Tag { tag_type: TagType::Primary, size: TagSize::Default, "Default" }
                Tag { tag_type: TagType::Primary, size: TagSize::Small,   "Small" }
            }
        }
    }
}

// ============================================================
// 7. Card 组件
// ============================================================
#[component]
fn CardSection() -> Element {
    rsx! {
        Card {
            header: Some("Card 组件".to_string()),
            shadow: "always".to_string(),

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",

                Card {
                    header: Some("Always Shadow".to_string()),
                    shadow: "always".to_string(),
                    div { style: "padding: 20px 0; text-align: center; color: #909399;", "shadow: always" }
                }
                Card {
                    header: Some("Hover Shadow".to_string()),
                    shadow: "hover".to_string(),
                    div { style: "padding: 20px 0; text-align: center; color: #909399;", "shadow: hover" }
                }
                Card {
                    header: Some("Never Shadow".to_string()),
                    shadow: "never".to_string(),
                    div { style: "padding: 20px 0; text-align: center; color: #909399;", "shadow: never" }
                }
            }
        }
    }
}

// ============================================================
// 8. Dialog 组件
// ============================================================
#[component]
fn DialogSection() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        Card {
            header: Some("Dialog 组件".to_string()),
            shadow: "hover".to_string(),

            Button {
                variant: ButtonVariant::Primary,
                on_click: move |_: MouseEvent| visible.set(true),
                "打开对话框"
            }

            Dialog {
                visible: *visible.read(),
                title: Some("对话框标题".to_string()),
                width: "500px".to_string(),
                on_close: move |_| visible.set(false),

                div {
                    style: "padding: 20px 0;",
                    "这是一个对话框内容区域。点击关闭按钮或遮罩层可以关闭对话框。"
                }

                div {
                    style: "text-align: right; margin-top: 20px;",
                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: move |_: MouseEvent| visible.set(false),
                        "确定"
                    }
                }
            }
        }
    }
}

// ============================================================
// 9. Table 组件（排序 + 行点击）
// ============================================================
#[component]
fn TableSection() -> Element {
    let mut sort_key = use_signal(|| None::<String>);
    let mut sort_order = use_signal(|| SortOrder::None);
    let mut clicked_row = use_signal(|| None::<usize>);

    let sk = sort_key();
    let so = sort_order();
    let cr = clicked_row();

    let columns = vec![
        TableColumn {
            title: "姓名".to_string(),
            key: "name".to_string(),
            width: Some("200px".to_string()),
            sortable: true,
            fixed: None,
        },
        TableColumn {
            title: "年龄".to_string(),
            key: "age".to_string(),
            width: None,
            sortable: true,
            fixed: None,
        },
        TableColumn {
            title: "城市".to_string(),
            key: "city".to_string(),
            width: None,
            sortable: false,
            fixed: None,
        },
    ];

    let data = vec![
        HashMap::from([
            ("name".to_string(), "张三".to_string()),
            ("age".to_string(), "30".to_string()),
            ("city".to_string(), "北京".to_string()),
        ]),
        HashMap::from([
            ("name".to_string(), "李四".to_string()),
            ("age".to_string(), "25".to_string()),
            ("city".to_string(), "上海".to_string()),
        ]),
        HashMap::from([
            ("name".to_string(), "王五".to_string()),
            ("age".to_string(), "35".to_string()),
            ("city".to_string(), "广州".to_string()),
        ]),
    ];

    let clicked_info: String = cr.map(|i| format!("第 {} 行", i + 1)).unwrap_or_else(|| "未点击".to_string());

    rsx! {
        Card {
            header: Some("Table 组件（排序 + 行点击）".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "margin-bottom: 12px; color: #606266; font-size: 14px;",
                "排序: {sk:?} {so:?} | 点击行: {clicked_info}"
            }

            Table {
                columns: columns,
                data: data,
                stripe: true,
                border: true,
                highlight_current_row: true,
                sort_key: sk,
                sort_order: so,
                on_sort_change: move |(key, order): (String, SortOrder)| {
                    sort_key.set(Some(key));
                    sort_order.set(order);
                },
                on_row_click: move |idx: usize| {
                    clicked_row.set(Some(idx));
                },
            }
        }
    }
}

// ============================================================
// 10. Tree 组件（展开/折叠 + 复选框）
// ============================================================
#[component]
fn TreeSection() -> Element {
    let mut expanded_keys = use_signal(|| vec!["Level 1".to_string()]);
    let mut checked_keys = use_signal(|| vec!["Level 2-1".to_string()]);
    let mut clicked_node = use_signal(|| String::new());

    let ek = expanded_keys();
    let ck = checked_keys();
    let cn = clicked_node();

    let tree_data = vec![
        TreeNodeData::new("Level 1")
            .child(TreeNodeData::new("Level 2-1"))
            .child(TreeNodeData::new("Level 2-2")
                .child(TreeNodeData::new("Level 3-1"))
            ),
        TreeNodeData::new("Level 1-2")
            .child(TreeNodeData::new("Level 2-3")),
    ];

    let ek_clone = ek.clone();
    let ck_clone = ck.clone();

    rsx! {
        Card {
            header: Some("Tree 组件（展开/折叠 + 复选框）".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "margin-bottom: 12px; color: #606266; font-size: 14px;",
                "展开: {ek:?} | 选中: {ck:?} | 点击: {cn}"
            }

            Tree {
                data: tree_data,
                show_checkbox: true,
                highlight_current: true,
                expand_on_click_node: false,
                expanded_keys: ek,
                checked_keys: ck,
                on_node_click: move |label: String| {
                    clicked_node.set(label);
                },
                on_node_expand: move |(label, expanded): (String, bool)| {
                    let mut keys = ek_clone.clone();
                    if expanded {
                        if !keys.contains(&label) {
                            keys.push(label);
                        }
                    } else {
                        keys.retain(|k| k != &label);
                    }
                    expanded_keys.set(keys);
                },
                on_node_check: move |(label, checked): (String, bool)| {
                    let mut keys = ck_clone.clone();
                    if checked {
                        if !keys.contains(&label) {
                            keys.push(label);
                        }
                    } else {
                        keys.retain(|k| k != &label);
                    }
                    checked_keys.set(keys);
                },
            }
        }
    }
}

// ============================================================
// 11. Cascader 组件
// ============================================================
#[component]
fn CascaderSection() -> Element {
    let mut path = use_signal(|| vec![]);
    let path_val = path();
    let path_display: String = path_val.join(" / ");

    let options = vec![
        CascaderOption::new("zhejiang", "浙江")
            .child(CascaderOption::new("hangzhou", "杭州")
                .child(CascaderOption::new("xihu", "西湖区")))
            .child(CascaderOption::new("ningbo", "宁波")),
        CascaderOption::new("jiangsu", "江苏")
            .child(CascaderOption::new("nanjing", "南京"))
            .child(CascaderOption::new("suzhou", "苏州")),
    ];

    rsx! {
        Card {
            header: Some("Cascader 组件（级联选择）".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "max-width: 400px;",

                Cascader {
                    options: options,
                    model_value: path_val,
                    placeholder: "请选择地区".to_string(),
                    clearable: true,
                    on_change: move |p: Vec<String>| {
                        path.set(p);
                    },
                }

                div {
                    style: "color: #606266; font-size: 14px; margin-top: 8px;",
                    "选中路径: {path_display}"
                }
            }
        }
    }
}

// ============================================================
// 12. Transfer 组件
// ============================================================
#[component]
fn TransferSection() -> Element {
    let mut right_keys = use_signal(|| vec!["2".to_string()]);
    let rk = right_keys();
    let right_count = rk.len();

    let data = vec![
        TransferItem::new("1", "选项一"),
        TransferItem::new("2", "选项二"),
        TransferItem::new("3", "选项三"),
        TransferItem::new("4", "选项四"),
        TransferItem::new("5", "选项五"),
    ];

    rsx! {
        Card {
            header: Some("Transfer 组件（穿梭框）".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "margin-bottom: 12px; color: #606266; font-size: 14px;",
                "右侧已选: {right_count} 项"
            }

            Transfer {
                data: data,
                model_value: rk,
                titles: vec!["可选列表".to_string(), "已选列表".to_string()],
                on_change: move |keys: Vec<String>| {
                    right_keys.set(keys);
                },
            }
        }
    }
}

// ============================================================
// 13. Progress 组件
// ============================================================
#[component]
fn ProgressSection() -> Element {
    rsx! {
        Card {
            header: Some("Progress 组件".to_string()),
            shadow: "hover".to_string(),

            div {
                style: "display: flex; flex-direction: column; gap: 16px;",

                Progress { percentage: 30 }
                Progress { percentage: 60 }
                Progress { percentage: 100, status: ProgressStatus::Success }
                Progress { percentage: 45, status: ProgressStatus::Warning }
                Progress { percentage: 20, status: ProgressStatus::Exception }
            }
        }
    }
}
