# Dioxus Element Plus

<a href="https://github.com/pauljohn21/dioxus-element-plug">
  <img src="https://img.shields.io/github/stars/pauljohn21/dioxus-element-plug?style=social" alt="GitHub stars">
  <img src="https://img.shields.io/github/forks/pauljohn21/dioxus-element-plug?style=social" alt="GitHub forks">
  <img src="https://img.shields.io/github/issues/pauljohn21/dioxus-element-plug" alt="GitHub issues">
  <img src="https://img.shields.io/github/license/pauljohn21/dioxus-element-plug" alt="GitHub license">
</a>

<a href="https://crates.io/crates/dioxus-element-plug">
  <img src="https://img.shields.io/crates/v/dioxus-element-plug.svg" alt="Crates.io version">
  <img src="https://img.shields.io/crates/d/dioxus-element-plug.svg" alt="Crates.io downloads">
  <img src="https://img.shields.io/crates/l/dioxus-element-plug.svg" alt="Crates.io license">
</a>

<a href="https://docs.rs/dioxus-element-plug">
  <img src="https://docs.rs/dioxus-element-plug/badge.svg" alt="Documentation">
</a>

[English](README.md) | **简体中文**

> 为 Dioxus 0.7 提供 Element Plus 风格的 UI 组件库，使用纯 Rust 生成样式，零运行时开销。

**项目地址：[pauljohn21/dioxus-element-plug](https://github.com/pauljohn21/dioxus-element-plug)**

---

## 特性

- 🎨 **纯 Rust 样式系统** — 编译时生成 CSS，零运行时开销
- 🦀 **Rust 原生组件** — 基于 Dioxus 0.7 构建的类型安全组件
- 📦 **107+ 组件** — 完整覆盖 Element Plus 全部组件类别
- 🎯 **受控组件模式** — 父组件持有状态，通过 `EventHandler` 回调通信
- 📱 **响应式设计** — 支持移动端友好的 24 栅格布局系统
- ⚡ **零依赖样式** — 无需 SCSS 运行时编译
- 🌙 **内置深色模式** — `Theme::dark()` 和 `Theme::light()` 预设
- 🔣 **Element Plus 图标** — 可选的 `icons` 特性，提供 137+ SVG 图标

## 快速开始

### 1. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-element-plug = "0.3"
```

或直接从 GitHub 引用：

```toml
dioxus-element-plug = { git = "https://github.com/pauljohn21/dioxus-element-plug.git" }
```

#### 特性标志

- `icons`（默认）— 通过 `element-icons` crate 启用 Element Plus 图标支持
- `web` — Web 平台支持
- `server` — 服务端渲染支持

如需禁用图标：

```toml
dioxus-element-plug = { version = "0.3", default-features = false, features = ["web"] }
```

### 2. 生成样式

**方式 A：纯 Rust CSS 生成（推荐）**

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        style { "{styles}" }
        Button { variant: ButtonVariant::Primary, "点击我" }
    }
}
```

按需生成样式（减少最终包体积）：

> **注意**：`generate_styles_for_components()` 在 0.3.0 中已**废弃**——当前返回完整样式表
> （114 个组件）。按组件 tree-shaking 将在 0.4.0 重新提供。目前请使用 `generate_complete_styles()`：

```rust
let styles = CompleteStyleManager::new().generate_complete_styles();
```


### 3. 使用组件

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        style { "{styles}" }

        div {
            style: "padding: 24px; background-color: #f5f7fa; min-height: 100vh;",

            h1 { "我的应用" }

            Card {
                h2 { "欢迎！" }
                Button { variant: ButtonVariant::Primary, "点击我" }
                Input {
                    input_type: InputType::Text,
                    placeholder: Some("请输入文字...".to_string()),
                    size: InputSize::Medium,
                }
            }

            Row {
                Col { span: 12, p { "半宽列" } }
            }
        }
    }
}
```

## 可用组件

### 布局组件

| 组件 | 说明 |
|------|------|
| `Container` `Header` `Main` `Footer` `Aside` | 页面布局结构 |
| `Row` / `Col` | 24 栅格响应式布局系统 |
| `Space` | 弹性间距组件 |

### 表单组件

| 组件 | 关键 Props | 事件回调 |
|------|-----------|----------|
| `Button` | `variant: ButtonVariant`, `size: Option<ButtonSize>`, `disabled` | `on_click: EventHandler<MouseEvent>` |
| `Input` | `value: Option<String>`, `input_type: InputType`, `size: InputSize` | `on_change: EventHandler<Event<FormData>>` |
| `Select` | `model_value: Option<String>`, `options: Vec<SelectOption>`, `placeholder: String` | `on_change: EventHandler<String>` |
| `Switch` | `model_value: bool`, `size: SwitchSize` | `on_change: EventHandler<bool>` |
| `Checkbox` | `model_value: bool`, `border: bool` | `on_change: EventHandler<bool>` |
| `Radio` | `model_value: bool`, `border: bool` | `on_change: EventHandler<bool>` |
| `Rate` | `model_value: u32`, `max: u32` | `on_change: EventHandler<u32>` |
| `Slider` | `model_value: u32`, `min: u32`, `max: u32` | `on_change: EventHandler<u32>` |
| `InputNumber` | `model_value: i64`, `min: i64`, `max: i64` | `on_change: EventHandler<i64>` |
| `Form` / `FormItem` | `label`, `required` | — |

### 数据展示组件

| 组件 | 说明 |
|------|------|
| `Table` | 可排序、可筛选的数据表格，支持 `TableColumn` 定义列 |
| `Tree` | 树形控件，支持展开/折叠/复选框，使用 `TreeNodeData` 构建数据 |
| `Card` | 内容容器，支持头部和阴影 |
| `Tag` | 可关闭标签，支持 `TagType`、`TagSize`、`TagEffect` 变体 |
| `Progress` | 进度条，支持线形/圆形/仪表盘三种类型 |
| `Badge` | 徽章，支持数字/圆点显示 |
| `Avatar` | 用户头像 |
| `Collapse` / `CollapseItem` | 折叠面板 |
| `Descriptions` | 结构化数据展示 |

### 导航组件

| 组件 | 说明 |
|------|------|
| `Menu` / `MenuItem` / `SubMenu` | 水平/垂直导航菜单 |
| `Tabs` | 标签页 |
| `Breadcrumb` / `BreadcrumbItem` | 面包屑导航 |
| `Pagination` | 分页控件 |
| `Steps` / `Step` | 步骤条 |

### 反馈组件

| 组件 | 说明 |
|------|------|
| `Dialog` | 模态对话框（通过 `visible` 属性控制） |
| `Drawer` | 抽屉面板 |
| `Alert` | 警告提示 |
| `Message` | 消息提示 |
| `Notification` | 通知 |
| `Tooltip` / `Popover` / `Popconfirm` | 浮层提示 |

## 组件示例

### 按钮

```rust
use dioxus_element_plug::prelude::*;

rsx! {
    div {
        style: "display: flex; gap: 12px; flex-wrap: wrap;",

        Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "主要按钮" }
        Button { variant: ButtonVariant::Success, "成功" }
        Button { variant: ButtonVariant::Warning, "警告" }
        Button { variant: ButtonVariant::Danger, "危险" }
        Button { variant: ButtonVariant::Info, size: Some(ButtonSize::Small), "信息" }
    }
}
```

### 受控组件（Switch）

所有交互组件均采用受控组件模式——状态由父组件通过 `use_signal` 持有，通过 props 传入，通过 `EventHandler` 回调通知变更：

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let mut switch_on = use_signal(|| false);

    rsx! {
        Switch {
            model_value: switch_on(),           // 父组件持有状态
            on_change: move |v: bool| switch_on.set(v),  // 通知父组件
            active_text: Some("开".to_string()),
            inactive_text: Some("关".to_string()),
        }
    }
}
```

### 表格

```rust
use std::collections::HashMap;
use dioxus_element_plug::prelude::*;
use dioxus_element_plug::components::table::{TableColumn, SortOrder};

rsx! {
    Table {
        columns: vec![
            TableColumn { title: "姓名".into(), key: "name".into(), width: None, sortable: true, fixed: None },
            TableColumn { title: "年龄".into(), key: "age".into(), width: None, sortable: true, fixed: None },
        ],
        data: vec![
            HashMap::from([("name".into(), "张三".into()), ("age".into(), "28".into())]),
            HashMap::from([("name".into(), "李四".into()), ("age".into(), "32".into())]),
        ],
        stripe: true,
        border: true,
    }
}
```

### 栅格布局

```rust
use dioxus_element_plug::prelude::*;

rsx! {
    Container {
        direction: Some(ContainerDirection::Vertical),

        Header {
            height: "60px".to_string(),
            h1 { "我的应用" }
        }

        Container {
            Aside {
                width: "200px".to_string(),
                "侧边栏"
            }

            Main {
                Row {
                    gutter: 20,

                    Col { span: 12,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px;",
                            "列 1 (span=12)"
                        }
                    }

                    Col { span: 12,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px;",
                            "列 2 (span=12)"
                        }
                    }
                }
            }
        }

        Footer { "页脚" }
    }
}
```

## 主题定制

自 0.3.0 起，`Theme` 包含 50 个字段。可使用 `ThemeBuilder` 的 fluent API，
或使用结构体更新语法 `..Theme::default()` 只覆盖需要修改的字段：

```rust
use dioxus_element_plug::{ThemeBuilder, CompleteStyleManager};

let custom_theme = ThemeBuilder::new()
    .primary_color("#1890ff")
    .font_size_base("16px")
    .border_radius_base("6px")
    .build();

let styles = CompleteStyleManager::new()
    .with_theme(custom_theme)
    .generate_complete_styles();
```

或使用结构体更新语法：

```rust
use dioxus_element_plug::{Theme, CompleteStyleManager};

let dark = Theme {
    color_white: "#141414",
    color_black: "#ffffff",
    color_text_primary: "#E5EAF3",
    ..Theme::default()
};

let styles = CompleteStyleManager::new()
    .with_theme(dark)
    .generate_complete_styles();
```

## 项目状态

**生产就绪** — 107+ 组件已全部实现，采用纯 Rust 样式系统。

- ✅ 107+ 组件，使用 `#[component]` 宏定义
- ✅ 97 个组件文件，位于 `src/components/`
- ✅ 137+ 图标，通过 `element-icons` crate 提供
- ✅ 完整的 Element Plus 设计系统兼容
- ✅ 纯 Rust CSS 生成，零运行时开销
- ✅ 全部采用受控组件模式
- ✅ Dioxus 0.7 深度集成，零编译错误

## 项目结构

```
dioxus-element-plug/
├── src/
│   ├── components/     # 107+ Element Plus 风格组件（97 个文件）
│   ├── styles/          # 模块化 CSS 常量（颜色、间距、阴影等）
│   ├── style_system.rs  # 纯 Rust CSS 生成（Theme, CompleteStyleManager）
│   └── lib.rs           # Crate 入口 + prelude 模块
├── element-icons/       # Element Plus 图标 crate（137+ SVG 图标）
├── examples/
│   ├── component-showcase/  # 组件验证示例（13 个类别）
│   └── theme-switcher/      # 主题切换示例（5 种主题）
├── Cargo.toml
├── AGENTS.md           # AI Agent 开发指南
├── SKILL.md            # 技能文档
├── README.md           # English
└── README.zh-CN.md     # 简体中文（本文件）
```

## 常用命令

```bash
cargo check              # 编译检查
cargo test --lib         # 运行测试
cargo clippy             # 代码检查
cd examples/component-showcase && cargo check  # 验证示例
cd examples/theme-switcher && cargo check      # 验证主题示例
```

## 示例项目

- **[component-showcase](examples/component-showcase/)** — 组件验证示例，覆盖 13 个组件类别（Button, Input, Select, Switch, Alert, Tag, Card, Dialog, Table, Tree, Cascader, Transfer, Progress）
- **[theme-switcher](examples/theme-switcher/)** — 主题切换示例，包含 5 种主题（默认、暗色、绿色、紫色、橙色）

## 开源许可

MIT License — 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [Element Plus](https://element-plus.org/) — 现代设计系统和组件库
- [Dioxus](https://dioxuslabs.com/) — Rust 生态的现代化 React 风格框架

## 支持项目

如果这个项目对你的 Dioxus 应用有帮助，欢迎：

- ⭐ [Star 项目](https://github.com/pauljohn21/dioxus-element-plug)
- 🐛 [提交 Issue](https://github.com/pauljohn21/dioxus-element-plug/issues)
- 💬 [参与讨论](https://github.com/pauljohn21/dioxus-element-plug/discussions)
