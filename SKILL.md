---
name: dioxus-element-plug
description: Rust library for using Element Plus components with Dioxus 0.7 framework - 107+ components, pure Rust CSS generation, controlled component pattern
---

# Dioxus Element Plug - Skill Documentation

This skill provides comprehensive expertise for the **dioxus-element-plug** library v0.1.5 — a complete Element Plus component system for the Dioxus 0.7 framework, featuring 107+ `#[component]` components with pure Rust CSS generation.

## When to Use

Use this skill when you need expert assistance with:

- **Building Dioxus 0.7 applications** with Element Plus styled components
- **Using 107+ pre-built components** (Button, Input, Select, Table, Form, Dialog, etc.)
- **Pure Rust CSS generation** via `CompleteStyleManager` and `Theme`
- **Controlled component pattern** — parent owns state, passes via props + EventHandler
- **Element Plus theme-chalk CSS classes** — `el-button`, `el-button--primary`, `is-disabled`, etc.

## Core Architecture

### Controlled Component Pattern (CRITICAL)

ALL interactive components follow the **controlled component pattern**:

- State is owned by the parent and passed via props (`model_value`, `visible`, etc.)
- Changes are communicated upward via `EventHandler<T>` callbacks (`on_change`, `on_close`, etc.)
- Components NEVER hold their own state for data the parent should control

```rust
let mut switch_on = use_signal(|| false);
rsx! {
    Switch {
        model_value: switch_on(),
        on_change: move |v: bool| switch_on.set(v),
    }
}
```

### Component Definition Convention

Every component uses `#[derive(Props, Clone, PartialEq)]` + `#[component]`:

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyProps {
    pub children: Element,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn MyComponent(props: MyProps) -> Element { /* ... */ }
```

### CSS Class Naming

- Base: `el-{component}` (e.g., `el-button`, `el-input`)
- Modifier: `el-{component}--{variant}` (e.g., `el-button--primary`)
- State: `is-{state}` (e.g., `is-disabled`, `is-checked`)
- Part: `el-{component}__{part}` (e.g., `el-card__header`)

### rsx! Ownership Rules

1. **Pre-extract data from `props` before `rsx!`** — `props` is moved into `rsx!`
2. **Clone values that closures need to own** — each closure gets its own copy
3. **Use `as_ref()` for `Option<EventHandler>`** — `handler.as_ref().call(value)`
4. **Pre-compute style strings outside `rsx!`** — use `format!()` before the macro

## Component API Quick Reference

### Form Components

| Component | Key Props | Key Events |
|-----------|-----------|------------|
| Button | `variant: ButtonVariant`, `size: Option<ButtonSize>`, `disabled: bool` | `on_click: EventHandler<MouseEvent>` |
| Input | `value: Option<String>`, `input_type: InputType`, `size: InputSize` | `on_change: EventHandler<Event<FormData>>` |
| Select | `model_value: Option<String>`, `options: Vec<SelectOption>`, `placeholder: String` | `on_change: EventHandler<String>` |
| Switch | `model_value: bool`, `size: SwitchSize` | `on_change: EventHandler<bool>` |
| Checkbox | `model_value: bool`, `border: bool` | `on_change: EventHandler<bool>` |
| Radio | `model_value: bool`, `border: bool` | `on_change: EventHandler<bool>` |
| Rate | `model_value: u32`, `max: u32` | `on_change: EventHandler<u32>` |
| Slider | `model_value: u32`, `min: u32`, `max: u32` | `on_change: EventHandler<u32>` |
| InputNumber | `model_value: i64`, `min: i64`, `max: i64` | `on_change: EventHandler<i64>` |

### Display Components

| Component | Key Props |
|-----------|-----------|
| Table | `columns: Vec<TableColumn>`, `data: Vec<HashMap<String, String>>`, `stripe: bool` |
| Tree | `data: Vec<TreeNodeData>`, `show_checkbox: bool` |
| Tag | `tag_type: TagType`, `size: TagSize`, `effect: TagEffect` |
| Progress | `percentage: u32`, `progress_type: ProgressType`, `status: ProgressStatus` |
| Badge | `value: Option<String>`, `is_dot: bool`, `badge_type: BadgeType` |
| Alert | `title: String`, `alert_type: AlertType`, `closable: bool` |
| Card | `header: Option<String>`, `shadow: CardShadow` |

### Navigation Components

| Component | Key Props |
|-----------|-----------|
| Menu | `mode: MenuMode`, `collapse: bool` |
| Tabs | `model_value: String`, `type: TabsType` |
| Steps | `active: u32`, `direction: StepsDirection` |
| Pagination | `total: u32`, `page_size: u32`, `current_page: u32` |
| Dropdown | `trigger: DropdownTrigger` |

### Feedback Components

| Component | Key Props |
|-----------|-----------|
| Dialog | `visible: bool`, `title: Option<String>` |
| Drawer | `visible: bool`, `direction: DrawerDirection` |
| Tooltip | `content: String`, `placement: TooltipPlacement` |
| Message | `message: String`, `message_type: MessageType` |
| Notification | `title: String`, `message: Option<String>` |

### Layout Components

| Component | Key Props |
|-----------|-----------|
| Container | `direction: Option<ContainerDirection>` |
| Row | `gutter: u32`, `justify: Option<RowJustify>`, `align: Option<RowAlign>` |
| Col | `span: u32`, `offset: u32` |
| Space | `direction: SpaceDirection`, `size: SpaceSize` |

## CSS Strategy

Two approaches:

1. **CDN** (quick start):
```rust
rsx! {
    document::Link { rel: "stylesheet", href: "//unpkg.com/element-plus@2.4.4/dist/index.css" }
}
```

2. **Pure Rust CSS generation**:
```rust
let css = CompleteStyleManager::new().generate_complete_styles();
rsx! { style { "{css}" } }
```

## Import Pattern

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
```

## Project Stats

- **107+ components** with `#[component]` macro
- **96 component files** in `src/components/`
- Built on **Dioxus 0.7**

## Common Commands

- `cargo check` — Verify compilation
- `cargo test --lib` — Run tests
- `cargo clippy` — Lint check
