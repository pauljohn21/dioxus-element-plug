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

**English** | [简体中文](README.zh-CN.md)

> Element Plus UI components for Dioxus 0.7 with pure Rust styling — zero runtime overhead.

**Repository: [pauljohn21/dioxus-element-plug](https://github.com/pauljohn21/dioxus-element-plug)**

---

## Features

- 🎨 **Pure Rust styling** — Compile-time CSS generation, zero runtime overhead
- 🦀 **Rust-native components** — Type-safe components built for Dioxus 0.7
- 📦 **107+ components** — Complete Element Plus component library
- 🎯 **Controlled component pattern** — Parent owns state, communicates via `EventHandler`
- 📱 **Responsive design** — 24-column grid system
- ⚡ **Zero SCSS dependencies** — No runtime CSS compilation

## Quick Start

### 1. Add dependency

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-element-plug = "0.2.0"
```

Or from GitHub:

```toml
dioxus-element-plug = { git = "https://github.com/pauljohn21/dioxus-element-plug.git" }
```

### 2. Generate styles

**Option A: Pure Rust CSS generation (recommended)**

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        style { "{styles}" }
        Button { variant: ButtonVariant::Primary, "Click me!" }
    }
}
```

Tree-shaking — generate only what you need:

```rust
let styles = CompleteStyleManager::new()
    .generate_styles_for_components(&["button", "input", "alert"]);
```

### 3. Use components

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        style { "{styles}" }

        div {
            style: "padding: 24px; background-color: #f5f7fa; min-height: 100vh;",

            h1 { "My App" }

            Card {
                h2 { "Welcome!" }
                Button { variant: ButtonVariant::Primary, "Click me!" }
                Input {
                    input_type: InputType::Text,
                    placeholder: Some("Enter text...".to_string()),
                    size: InputSize::Medium,
                }
            }

            Row {
                Col { span: 12, p { "Half-width column" } }
            }
        }
    }
}
```

## Available Components

### Layout

| Component | Description |
|-----------|-------------|
| `Container` `Header` `Main` `Footer` `Aside` | Page layout structure |
| `Row` / `Col` | 24-column responsive grid system |
| `Space` | Flexible spacing component |

### Form

| Component | Key Props | Events |
|-----------|-----------|--------|
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

### Data Display

| Component | Description |
|-----------|-------------|
| `Table` | Sortable, filterable data table with `TableColumn` |
| `Tree` | Hierarchical tree with `TreeNodeData`, checkbox support |
| `Card` | Content container with header and shadow |
| `Tag` | Closable tags with `TagType`, `TagSize`, `TagEffect` |
| `Progress` | Line, circle, dashboard progress bars |
| `Badge` | Notification badges, dot mode |
| `Avatar` | User avatars |
| `Collapse` / `CollapseItem` | Collapsible panels |
| `Descriptions` | Structured data display |

### Navigation

| Component | Description |
|-----------|-------------|
| `Menu` / `MenuItem` / `SubMenu` | Horizontal / vertical navigation menu |
| `Tabs` | Tabbed content |
| `Breadcrumb` / `BreadcrumbItem` | Breadcrumb navigation |
| `Pagination` | Pagination control |
| `Steps` / `Step` | Step progress indicator |

### Feedback

| Component | Description |
|-----------|-------------|
| `Dialog` | Modal dialog (controlled via `visible` prop) |
| `Drawer` | Slide-out panel |
| `Alert` | Alert messages |
| `Message` | Toast messages |
| `Notification` | Notifications |
| `Tooltip` / `Popover` / `Popconfirm` | Floating overlays |

## Component Examples

### Buttons

```rust
use dioxus_element_plug::prelude::*;

rsx! {
    div {
        style: "display: flex; gap: 12px; flex-wrap: wrap;",

        Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Large), "Primary" }
        Button { variant: ButtonVariant::Success, "Success" }
        Button { variant: ButtonVariant::Warning, "Warning" }
        Button { variant: ButtonVariant::Danger, "Danger" }
        Button { variant: ButtonVariant::Info, size: Some(ButtonSize::Small), "Info" }
    }
}
```

### Controlled Component (Switch)

All interactive components follow the controlled pattern — parent owns state via `use_signal`, passes it through props, and receives changes via `EventHandler` callbacks:

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    let mut switch_on = use_signal(|| false);

    rsx! {
        Switch {
            model_value: switch_on(),
            on_change: move |v: bool| switch_on.set(v),
            active_text: Some("On".to_string()),
            inactive_text: Some("Off".to_string()),
        }
    }
}
```

### Table

```rust
use std::collections::HashMap;
use dioxus_element_plug::prelude::*;
use dioxus_element_plug::components::table::{TableColumn, SortOrder};

rsx! {
    Table {
        columns: vec![
            TableColumn { title: "Name".into(), key: "name".into(), width: None, sortable: true, fixed: None },
            TableColumn { title: "Age".into(), key: "age".into(), width: None, sortable: true, fixed: None },
        ],
        data: vec![
            HashMap::from([("name".into(), "John".into()), ("age".into(), "28".into())]),
            HashMap::from([("name".into(), "Jane".into()), ("age".into(), "32".into())]),
        ],
        stripe: true,
        border: true,
    }
}
```

### Grid Layout

```rust
use dioxus_element_plug::prelude::*;

rsx! {
    Container {
        direction: Some(ContainerDirection::Vertical),

        Header {
            height: "60px".to_string(),
            h1 { "My App" }
        }

        Container {
            Aside {
                width: "200px".to_string(),
                "Sidebar"
            }

            Main {
                Row {
                    gutter: 20,

                    Col { span: 12,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px;",
                            "Column 1 (span=12)"
                        }
                    }

                    Col { span: 12,
                        div {
                            style: "background: #f0f9ff; padding: 20px; border-radius: 4px;",
                            "Column 2 (span=12)"
                        }
                    }
                }
            }
        }

        Footer { "Footer" }
    }
}
```

## Theme Customization

```rust
use dioxus_element_plug::{Theme, CompleteStyleManager};

let custom_theme = Theme::new()
    .with_primary_color("#1890ff")
    .with_font_size("16px");

let styles = CompleteStyleManager::new()
    .with_theme(custom_theme)
    .generate_complete_styles();
```

## Project Status

**Production Ready** — 107+ components with pure Rust styling.

- ✅ 107+ components via `#[component]` macro
- ✅ 96 component files in `src/components/`
- ✅ Full Element Plus design system compatibility
- ✅ Pure Rust CSS generation (zero runtime overhead)
- ✅ Controlled component pattern throughout
- ✅ Dioxus 0.7 integration, zero compilation errors

## Project Structure

```
dioxus-element-plug/
├── src/
│   ├── components/     # 107+ Element Plus style components (96 files)
│   ├── styles/          # Modular CSS constants (colors, spacing, shadows, etc.)
│   ├── style_system.rs  # Pure Rust CSS generation (Theme, CompleteStyleManager)
│   └── lib.rs           # Crate entry point + prelude module
├── examples/
│   ├── component-showcase/  # Component verification (13 categories)
│   └── theme-switcher/      # Theme switching demo (5 themes)
├── Cargo.toml
├── AGENTS.md           # AI agent guidance
├── SKILL.md            # Skill documentation
├── README.md           # English (this file)
└── README.zh-CN.md     # 简体中文
```

## Commands

```bash
cargo check              # Verify compilation
cargo test --lib         # Run tests
cargo clippy             # Lint check
cd examples/component-showcase && cargo check  # Verify example
cd examples/theme-switcher && cargo check      # Verify theme example
```

## Examples

- **[component-showcase](examples/component-showcase/)** — 13 component categories (Button, Input, Select, Switch, Alert, Tag, Card, Dialog, Table, Tree, Cascader, Transfer, Progress)
- **[theme-switcher](examples/theme-switcher/)** — 5 themes (Default, Dark, Green, Purple, Orange)

## License

MIT License — see [LICENSE](LICENSE) file for details.

## Credits

- [Element Plus](https://element-plus.org/) — Modern design system and component library
- [Dioxus](https://dioxuslabs.com/) — Modern React-like framework for Rust

## Star & Support

If you find this project helpful, please consider:

- ⭐ [Star the Repository](https://github.com/pauljohn21/dioxus-element-plug)
- 🐛 [Report Issues](https://github.com/pauljohn21/dioxus-element-plug/issues)
- 💬 [Join Discussions](https://github.com/pauljohn21/dioxus-element-plug/discussions)
