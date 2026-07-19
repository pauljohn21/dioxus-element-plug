# AGENTS.md

Guidance for coding agents working in this repository.

## Project Overview

**dioxus-element-plug** (v0.2.2) is a Rust UI component library that brings Element Plus (Vue 3) design and CSS classes to the Dioxus 0.7 framework. It provides 107+ components styled with Element Plus `theme-chalk` CSS classes, rendered via Dioxus `#[component]` and `rsx!` macros. All CSS is generated in pure Rust — zero runtime overhead, no CDN or external CSS dependencies.

**Repository**: [gitee.com/pauljoihn21/dioxus-element-plug](https://gitee.com/pauljoihn21/dioxus-element-plug)

## Repository Map

```
dioxus-element-plug/
├── src/
│   ├── components/          # 107+ Element Plus style components (97 .rs files)
│   │   ├── mod.rs           # Module declarations + glob re-exports
│   │   ├── common.rs        # Shared utilities: ClassBuilder, style_str, fire_event
│   │   ├── button.rs        # Button, ButtonProps, ButtonVariant, ButtonSize
│   │   ├── input.rs         # Input, InputProps, InputType, InputSize (refactored with ClassBuilder)
│   │   ├── select.rs        # Select, SelectProps, SelectOption, SelectSize
│   │   ├── switch.rs        # Switch, SwitchProps, SwitchSize
│   │   ├── checkbox.rs      # Checkbox, CheckboxProps, CheckboxGroup, CheckboxButton
│   │   ├── radio.rs         # Radio, RadioProps, RadioGroup, RadioButton
│   │   ├── table.rs         # Table, TableProps, TableColumn, SortOrder
│   │   ├── tree.rs          # Tree, TreeProps, TreeNodeData
│   │   ├── cascader.rs      # Cascader, CascaderProps, CascaderOption
│   │   ├── transfer.rs      # Transfer, TransferProps, TransferItem
│   │   ├── tag.rs           # Tag, TagProps, TagType, TagSize, TagEffect
│   │   ├── progress.rs      # Progress, ProgressProps, ProgressType, ProgressStatus
│   │   ├── dialog.rs        # Dialog, DialogProps
│   │   ├── drawer.rs        # Drawer, DrawerProps, DrawerDirection
│   │   ├── menu.rs          # Menu, MenuItem, SubMenu, MenuItemGroup
│   │   ├── container.rs     # Container, Header, Footer, Main, Aside (all in one file)
│   │   ├── row.rs           # Row, Col, RowJustify, RowAlign (both in one file)
│   │   ├── steps.rs         # Steps, Step, StepStatus (both in one file)
│   │   ├── dropdown.rs      # Dropdown, DropdownMenu, DropdownItem
│   │   └── ...              # Other components follow the same pattern
│   ├── style_system.rs      # Pure Rust CSS generation (Theme, CompleteStyleManager)
│   ├── styles/              # Modular style constants (colors, spacing, shadows, etc.)
│   └── lib.rs               # Crate entry point + prelude module
├── examples/
│   ├── component-showcase/  # Verification example covering 13 component categories
│   └── theme-switcher/      # Theme switching example (5 themes)
├── AGENTS.md                # AI agent guidance (this file)
├── SKILL.md                 # Skill documentation for AI agents
├── README.md                # Project README (English)
├── README.zh-CN.md          # Project README (简体中文)
└── Cargo.toml
```

## Common Commands

- Build/check: `cargo check`
- Run tests: `cargo test --lib`
- Type check: `cargo clippy`
- Run example: `cd examples/component-showcase && cargo check`
- Run theme example: `cd examples/theme-switcher && cargo check`

## Architecture & Design Principles

### 1. Controlled Component Pattern (CRITICAL)

ALL interactive components follow the **controlled component pattern** consistent with Element Plus Vue:

- **State is owned by the parent** and passed down via props (e.g., `model_value`, `visible`, `current_page`)
- **Changes are communicated upward** via `EventHandler` callbacks (e.g., `on_change`, `on_close`, `on_click`)
- **Components never hold their own state** for data that the parent should control

```rust
// ✅ CORRECT: Parent owns the state
let mut switch_on = use_signal(|| false);

rsx! {
    Switch {
        model_value: switch_on(),        // State from parent
        on_change: move |v: bool| switch_on.set(v),  // Notify parent
    }
}

// ❌ WRONG: Never add internal state to controlled components
// Components should NOT contain `use_signal` for their primary value
```

> **Note**: UI-only state (like focus tracking for visual feedback) is acceptable via `use_signal` inside components. The rule applies to *data* state that the parent should control.

### 2. CSS Class Naming Convention

All components use Element Plus `theme-chalk` CSS class names:

- **Base class**: `el-{component}` (e.g., `el-button`, `el-input`, `el-table`)
- **Modifier classes**: `el-{component}--{modifier}` (e.g., `el-button--primary`, `el-input--large`)
- **State classes**: `is-{state}` (e.g., `is-disabled`, `is-checked`, `is-active`, `is-focus`)
- **Part classes**: `el-{component}__{part}` (e.g., `el-card__header`, `el-input__inner`, `el-input__wrapper`)

Every enum variant that maps to a CSS class has an `as_class()` method returning `&'static str`.

### 3. Common Utilities (`src/components/common.rs`)

Shared utilities to reduce boilerplate across components. All new and refactored components should use these:

#### `ClassBuilder` — Fluent CSS class builder

Eliminates the repetitive `vec![]` → `push` → `join(" ")` pattern:

```rust
use crate::components::common::ClassBuilder;

let class = ClassBuilder::new("el-input")
    .add_class("el-input--large")       // Unconditional (skipped if empty)
    .add_if("is-disabled", disabled)     // Conditional
    .add_if("is-error", has_error)       // Conditional
    .add_opt(props.class.as_ref())       // From Option<&String>
    .add_opt_str(Some("extra"))          // From Option<&str>
    .build();                            // → "el-input el-input--large is-disabled extra"
```

#### `style_str` — Extract style string

```rust
use crate::components::common::style_str;

let s = style_str(&props.style);  // Returns String, empty if None
```

#### `fire_event` — Call optional event handler

```rust
use crate::components::common::fire_event;

// Inside rsx! closure:
oninput: move |event| fire_event(&on_input, event),
```

### 4. Props Struct Convention

All component props follow this pattern:

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    // Required: children element
    pub children: Element,

    // Variant/type enum with Default
    #[props(default = MyType::Default)]
    pub my_type: MyType,

    // Boolean flags default to false
    #[props(default = false)]
    pub disabled: bool,

    // Optional strings
    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,

    // Event handlers are Optional<EventHandler<T>>
    #[props(default)]
    pub on_change: Option<EventHandler<SomeType>>,
}
```

**Key rules**:
- `children: Element` is required (with `#[props(default)]` only for components where children is optional)
- `class` and `style` are always `Option<String>` with `#[props(default)]`
- Event handlers are always `Option<EventHandler<T>>` with `#[props(default)]`
- Boolean props use `#[props(default = false)]`
- String props that have defaults use `#[props(default = "...".to_string())]`

### 5. Component Implementation Template (using ClassBuilder)

```rust
use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Type/variant enum
#[derive(Clone, PartialEq)]
pub enum MyType {
    Default,
    Primary,
}

impl MyType {
    pub fn as_class(&self) -> &'static str {
        match self {
            MyType::Default => "",
            MyType::Primary => "el-my-component--primary",
        }
    }
}

/// Props struct
#[derive(Props, Clone, PartialEq)]
pub struct MyComponentProps {
    pub children: Element,
    #[props(default = MyType::Default)]
    pub my_type: MyType,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

/// Component function
#[component]
pub fn MyComponent(props: MyComponentProps) -> Element {
    // Build class string using ClassBuilder
    let class_string = ClassBuilder::new("el-my-component")
        .add_class(props.my_type.as_class())
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_click = props.on_click;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            onclick: move |e: MouseEvent| {
                fire_event(&on_click, e);
            },
            {props.children}
        }
    }
}
```

## rsx! Ownership Rules (CRITICAL)

### Rule 1: Pre-extract data from `props` before `rsx!`

`props` is moved into `rsx!`. You cannot borrow `props` fields in closures inside `rsx!`.

```rust
// ❌ WRONG: props.options is borrowed inside a closure in rsx!
for option in props.options.iter() {
    div {
        onclick: move |_| handler.call(option.value.clone()), // ERROR: props dropped
    }
}

// ✅ CORRECT: Pre-extract into a Vec of owned data
let option_data: Vec<(String, String, bool)> = props.options.iter()
    .map(|o| (o.value.clone(), o.label.clone(), o.disabled))
    .collect();

rsx! {
    for (value, label, is_disabled) in option_data.into_iter() {
        div {
            onclick: move |_| handler.call(value.clone()),
            "{label}"
        }
    }
}
```

### Rule 2: Clone values that closures need to own

Each closure in `for` loops gets its own copy. Use `.clone()` on values before the closure:

```rust
// ❌ WRONG: String moved into first closure, unavailable for second
for item in items {
    div { onclick: move |_| handler.call(item.key), }  // item.key moved
}

// ✅ CORRECT: Each iteration produces owned values
for (key, label) in items.into_iter() {
    div { onclick: move |_| handler.call(key), "{label}" }
}
```

### Rule 3: Use `fire_event` or `as_ref()` for `Option<EventHandler>`

```rust
use crate::components::common::fire_event;

let on_close = props.on_close;

// ✅ Pattern A: Using fire_event helper (preferred)
onclick: move |_| fire_event(&on_close, ()),

// ✅ Pattern B: Manual as_ref()
if let Some(handler) = on_close.as_ref() {
    handler.call(());
}
```

### Rule 4: Pre-compute style strings outside `rsx!`

```rust
use crate::components::common::style_str;

// ✅ Build the style string before rsx!
let outer_style = format!(
    "position: relative; display: inline-block;{}",
    style_str(&props.style)
);

rsx! {
    div { style: "{outer_style}", }
}
```

## Component API Reference

### Core Form Components

#### Button (`src/components/button.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| children | `Element` | required | Button content |
| variant | `ButtonVariant` | `Default` | Primary, Success, Warning, Danger, Info |
| size | `Option<ButtonSize>` | `None` | Large, Medium, Small, Mini |
| disabled | `bool` | `false` | Disable the button |
| round | `bool` | `false` | Round button |
| circle | `bool` | `false` | Circle button |
| loading | `bool` | `false` | Loading state |
| icon | `Option<String>` | `None` | Icon CSS class |
| button_type | `String` | `"button"` | Native button type |
| on_click | `Option<EventHandler<MouseEvent>>` | `None` | Click handler (**not** `onclick`) |
| class | `Option<String>` | `None` | Extra CSS classes |
| style | `Option<String>` | `None` | Inline styles |

**Usage**:
```rust
Button {
    variant: ButtonVariant::Primary,
    size: Some(ButtonSize::Large),
    on_click: move |_: MouseEvent| { /* ... */ },
    "Submit"
}
```

Also provides: `OutlineButton`, `TextButton`, `LinkButton` variants.

#### Input (`src/components/input.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| value | `Option<String>` | `None` | Current value |
| placeholder | `Option<String>` | `None` | Placeholder text |
| input_type | `InputType` | `Text` | Text, Password, Email, Url, Number, Tel, Search, Textarea |
| size | `Option<InputSize>` | `None` | Large, Medium, Small, Mini |
| disabled | `bool` | `false` | |
| readonly | `bool` | `false` | |
| clearable | `bool` | `false` | |
| show_password | `bool` | `false` | Password toggle (for password inputs) |
| error | `bool` | `false` | Error state |
| label | `Option<String>` | `None` | Input label |
| prefix_icon | `Option<String>` | `None` | Prefix icon CSS class |
| suffix_icon | `Option<String>` | `None` | Suffix icon CSS class |
| maxlength | `Option<u32>` | `None` | |
| minlength | `Option<u32>` | `None` | |
| autofocus | `bool` | `false` | |
| name | `Option<String>` | `None` | |
| id | `Option<String>` | `None` | |
| on_input | `Option<EventHandler<FormEvent>>` | `None` | Value change on keystroke |
| on_change | `Option<EventHandler<FormEvent>>` | `None` | Value change on blur |
| on_focus | `Option<EventHandler<FocusEvent>>` | `None` | Focus handler |
| on_blur | `Option<EventHandler<FocusEvent>>` | `None` | Blur handler |
| on_keydown | `Option<EventHandler<KeyboardEvent>>` | `None` | Key down handler |

Also provides: `SearchInput`, `PasswordInput` variants.

**Note**: Input uses `ClassBuilder` for class construction and `fire_event` for event handling. It tracks focus state internally via `use_signal` for `is-focus` CSS class on `el-input__wrapper`.

#### Select (`src/components/select.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `Option<String>` | `None` | Selected value |
| options | `Vec<SelectOption>` | `vec![]` | Options list |
| placeholder | `String` | `"Select"` | **Not** `Option<String>` |
| disabled | `bool` | `false` | |
| clearable | `bool` | `false` | |
| on_change | `Option<EventHandler<String>>` | `None` | **EventHandler<String>**, not `Event<FormData>` |

**SelectOption**:
```rust
SelectOption::new("value", "Label")
SelectOption::new("value", "Label").disabled(true)
```

#### Switch (`src/components/switch.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `bool` | `false` | On/off state |
| disabled | `bool` | `false` | |
| loading | `bool` | `false` | |
| size | `SwitchSize` | `Default` | Large, Default, Small |
| active_text | `Option<String>` | `None` | Text when on |
| inactive_text | `Option<String>` | `None` | Text when off |
| active_color | `String` | `"#409EFF"` | |
| inactive_color | `String` | `"#C0CCDA"` | |
| on_change | `Option<EventHandler<bool>>` | `None` | |

#### Checkbox (`src/components/checkbox.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `bool` | `false` | Checked state |
| disabled | `bool` | `false` | |
| border | `bool` | `false` | |
| indeterminate | `bool` | `false` | |
| on_change | `Option<EventHandler<bool>>` | `None` | |

#### Radio (`src/components/radio.rs`)

Same pattern as Checkbox: `model_value: bool`, `on_change: Option<EventHandler<bool>>`.

#### Rate (`src/components/rate.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `u32` | `0` | Current rating |
| max | `u32` | `5` | Maximum stars |
| allow_half | `bool` | `false` | |
| show_score | `bool` | `false` | |
| on_change | `Option<EventHandler<u32>>` | `None` | |

#### Slider (`src/components/slider.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `u32` | `0` | Current value |
| min | `u32` | `0` | |
| max | `u32` | `100` | |
| step | `u32` | `1` | |
| disabled | `bool` | `false` | |
| on_change | `Option<EventHandler<u32>>` | `None` | |

#### InputNumber (`src/components/input_number.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| model_value | `i64` | `0` | Current value |
| min | `i64` | `i64::MIN` | |
| max | `i64` | `i64::MAX` | |
| step | `i64` | `1` | |
| disabled | `bool` | `false` | |
| on_change | `Option<EventHandler<i64>>` | `None` | |

### Display Components

#### Table (`src/components/table.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| columns | `Vec<TableColumn>` | required | Column definitions |
| data | `Vec<HashMap<String, String>>` | required | Row data |
| stripe | `bool` | `false` | Zebra striping |
| border | `bool` | `false` | Borders |
| highlight_current_row | `bool` | `false` | |
| sort_key | `Option<String>` | `None` | Controlled sort column |
| sort_order | `SortOrder` | `None` | Ascending, Descending, None |
| on_sort_change | `Option<EventHandler<(String, SortOrder)>>` | `None` | |
| on_row_click | `Option<EventHandler<usize>>` | `None` | |

**TableColumn** struct:
```rust
TableColumn {
    title: String,
    key: String,
    width: Option<String>,
    sortable: bool,
    fixed: Option<String>,
}
```

#### Tree (`src/components/tree.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| data | `Vec<TreeNodeData>` | required | Tree data |
| show_checkbox | `bool` | `false` | |
| highlight_current | `bool` | `false` | |
| expanded_keys | `Vec<String>` | `vec![]` | Controlled expanded nodes |
| checked_keys | `Vec<String>` | `vec![]` | Controlled checked nodes |
| on_node_click | `Option<EventHandler<String>>` | `None` | |
| on_node_expand | `Option<EventHandler<(String, bool)>>` | `None` | |
| on_node_check | `Option<EventHandler<(String, bool)>>` | `None` | |

**TreeNodeData** builder:
```rust
TreeNodeData::new("Label")
    .child(TreeNodeData::new("Child 1"))
    .child(TreeNodeData::new("Child 2"))
```

#### Tag (`src/components/tag.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| children | `Element` | required | Tag content |
| tag_type | `TagType` | `Primary` | Primary, Success, Info, Warning, Danger |
| size | `TagSize` | `Default` | Large, Default, Small |
| effect | `TagEffect` | `Light` | Dark, Light, Plain |
| closable | `bool` | `false` | |
| on_close | `Option<EventHandler<MouseEvent>>` | `None` | |

#### Progress (`src/components/progress.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| percentage | `u32` | `0` | 0-100 |
| progress_type | `ProgressType` | `Line` | Line, Circle, Dashboard |
| status | `ProgressStatus` | `Default` | Default, Success, Warning, Exception |
| stroke_width | `u32` | `6` | |
| show_text | `bool` | `true` | |

#### Badge (`src/components/badge.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| children | `Element` | required | Element the badge is attached to |
| value | `Option<String>` | `None` | Badge display value |
| max | `u32` | `99` | Shows `{max}+` when exceeded |
| is_dot | `bool` | `false` | Show dot instead of value |
| hidden | `bool` | `false` | |
| badge_type | `BadgeType` | `Danger` | Primary, Success, Warning, Info, Danger |

### Navigation Components

#### Menu (`src/components/menu.rs`)

Contains: `Menu`, `MenuItem`, `SubMenu`, `MenuItemGroup`

| Menu Prop | Type | Default | Description |
|-----------|------|---------|-------------|
| children | `Element` | required | Menu items |
| mode | `MenuMode` | `Vertical` | Vertical, Horizontal |
| collapse | `bool` | `false` | Collapse sidebar mode |

```rust
Menu {
    mode: MenuMode::Horizontal,
    MenuItem { index: Some("1".to_string()), "Home" }
    SubMenu { title: Some("More".to_string()),
        MenuItem { index: Some("2".to_string()), "About" }
    }
}
```

#### Steps (`src/components/steps.rs`)

Contains: `Steps`, `Step`

| Steps Prop | Type | Default | Description |
|------------|------|---------|-------------|
| active | `u32` | `0` | Current step index |
| direction | `StepsDirection` | `Horizontal` | |
| process_status | `StepStatus` | `Process` | Wait, Process, Finish, Error, Success |

```rust
Steps {
    active: 1,
    Step { title: Some("Step 1".to_string()), "First" }
    Step { title: Some("Step 2".to_string()), status: StepStatus::Process, "Second" }
}
```

#### Pagination (`src/components/pagination.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| total | `u32` | `0` | Total item count |
| page_size | `u32` | `10` | Items per page |
| current_page | `u32` | `1` | Current page (1-indexed) |
| on_current_change | `Option<EventHandler<u32>>` | `None` | Page change handler |

### Feedback Components

#### Dialog (`src/components/dialog.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| children | `Element` | (optional) | Dialog body |
| visible | `bool` | `false` | Show/hide (controlled) |
| title | `Option<String>` | `None` | |
| width | `String` | `"50%"` | |
| modal | `bool` | `true` | |
| close_on_click_modal | `bool` | `true` | |
| show_close | `bool` | `true` | |
| on_close | `Option<EventHandler<()>>` | `None` | |

#### Drawer (`src/components/drawer.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| visible | `bool` | `false` | |
| direction | `DrawerDirection` | `Right` | Left, Right, Top, Bottom |
| size | `String` | `"30%"` | |
| on_close | `Option<EventHandler<()>>` | `None` | |

#### Alert (`src/components/alert.rs`)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| title | `String` | required | Alert title |
| description | `Option<String>` | `None` | |
| alert_type | `AlertType` | required | Success, Warning, Info, Error |
| closable | `bool` | `true` | |
| show_icon | `bool` | `true` | |
| center | `bool` | `false` | |

#### Tooltip/Popover/Popconfirm

| Component | Key Props | Notes |
|-----------|-----------|-------|
| Tooltip | `content`, `placement: TooltipPlacement`, `visible: Option<bool>` | |
| Popover | `content`, `title`, `placement: PopoverPlacement`, `visible` | |
| Popconfirm | `title`, `on_confirm`, `on_cancel`, `visible` | Uses `ButtonVariant` from button.rs |

#### Message/Notification

| Component | Key Props | Notes |
|-----------|-----------|-------|
| Message | `message`, `message_type: MessageType`, `visible`, `closable` | |
| Notification | `title`, `message`, `notification_type: NotificationType`, `closable` | |

### Layout Components

#### Container system (`src/components/container.rs`)

All in one file: `Container`, `Header`, `Footer`, `Main`, `Aside`

```rust
Container {
    direction: Some(ContainerDirection::Vertical),
    Header { height: "60px".to_string(), "Top Bar" }
    Container {
        Aside { width: "200px".to_string(), "Sidebar" }
        Main { "Content" }
    }
    Footer { "Bottom Bar" }
}
```

#### Row/Col (`src/components/row.rs`)

```rust
Row {
    gutter: 20,
    justify: Some(RowJustify::Center),
    align: Some(RowAlign::Middle),
    Col { span: 12, "Left" }
    Col { span: 12, "Right" }
}
```

## Adding a New Component

1. Create `src/components/{component_name}.rs`
2. Import common utilities: `use crate::components::common::{ClassBuilder, style_str, fire_event};`
3. Define the `#[derive(Props, Clone, PartialEq)]` struct: `{ComponentName}Props`
4. Implement the `#[component] pub fn {ComponentName}(props: {ComponentName}Props) -> Element` function
5. Use `ClassBuilder` for CSS class construction, `style_str` for styles, `fire_event` for events
6. Add `pub mod {component_name};` and `pub use {component_name}::*;` to `src/components/mod.rs`
7. Add `pub use crate::components::{component_name}::*;` to `src/lib.rs` prelude
8. Run `cargo check` to verify

### Component File Template (with ClassBuilder)

```rust
use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// {ComponentName} type/variant enum
#[derive(Clone, PartialEq)]
pub enum {ComponentName}Type {
    Default,
    Primary,
}

impl {ComponentName}Type {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "el-{component-name}--primary",
        }
    }
}

/// {ComponentName} props
#[derive(Props, Clone, PartialEq)]
pub struct {ComponentName}Props {
    pub children: Element,
    #[props(default = {ComponentName}Type::Default)]
    pub my_type: {ComponentName}Type,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub on_change: Option<EventHandler<()>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

/// {ComponentName} component
#[component]
pub fn {ComponentName}(props: {ComponentName}Props) -> Element {
    let class_string = ClassBuilder::new("el-{component-name}")
        .add_class(props.my_type.as_class())
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            onclick: move |_| {
                fire_event(&on_change, ());
            },
            {props.children}
        }
    }
}
```

## Common Pitfalls

### 1. Event handler prop naming

In Dioxus `rsx!`, native HTML events use lowercase (`onclick`, `onchange`). But our component props use snake_case (`on_click`, `on_change`). Make sure you use the correct name:

```rust
// ✅ Correct: Component prop
Button { on_click: move |_: MouseEvent| {}, "Click" }

// ❌ Wrong: This is the native HTML event, not the component prop
Button { onclick: move |_: MouseEvent| {}, "Click" }
```

### 2. String vs Option<String> for props

Some props are `String` (with a default), others are `Option<String>`:

```rust
// String with default - pass directly
Select { placeholder: "Choose".to_string(), }  // NOT Some("Choose".to_string())

// Option<String> - use Some() or leave empty
Input { placeholder: Some("Enter text".to_string()), }
```

### 3. EventHandler callback signatures

Different components use different callback signatures. Always check the prop type:

| Component | Event Prop | Callback Signature |
|-----------|-----------|-------------------|
| Button | `on_click` | `EventHandler<MouseEvent>` |
| Input | `on_input` / `on_change` | `EventHandler<FormEvent>` |
| Input | `on_focus` / `on_blur` | `EventHandler<FocusEvent>` |
| Input | `on_keydown` | `EventHandler<KeyboardEvent>` |
| Select | `on_change` | `EventHandler<String>` |
| Switch | `on_change` | `EventHandler<bool>` |
| Checkbox | `on_change` | `EventHandler<bool>` |
| Rate | `on_change` | `EventHandler<u32>` |
| Slider | `on_change` | `EventHandler<u32>` |
| InputNumber | `on_change` | `EventHandler<i64>` |
| Table | `on_sort_change` | `EventHandler<(String, SortOrder)>` |
| Table | `on_row_click` | `EventHandler<usize>` |
| Dialog | `on_close` | `EventHandler<()>` |
| Pagination | `on_current_change` | `EventHandler<u32>` |

### 4. Multi-component files

Some files define multiple related components:
- `container.rs` → `Container`, `Header`, `Footer`, `Main`, `Aside`
- `row.rs` → `Row`, `Col`
- `menu.rs` → `Menu`, `MenuItem`, `SubMenu`, `MenuItemGroup`
- `steps.rs` → `Steps`, `Step`
- `dropdown.rs` → `Dropdown`, `DropdownMenu`, `DropdownItem`
- `checkbox.rs` → `Checkbox`, `CheckboxGroup`, `CheckboxButton`
- `radio.rs` → `Radio`, `RadioGroup`, `RadioButton`
- `button.rs` → `Button`, `OutlineButton`, `TextButton`, `LinkButton`
- `input.rs` → `Input`, `SearchInput`, `PasswordInput`

Sub-components in separate files (like `header.rs`, `footer.rs`, `col.rs`, `step.rs`, etc.) simply re-export from the parent file:
```rust
// header.rs
pub use super::container::{Header, HeaderProps};
```

### 5. Placeholder components

Some components currently use a minimal placeholder implementation with only `children`, `class`, and `style` props. These need to be enhanced with proper Element Plus API when needed:

`Anchor`, `AnchorLink`, `Autocomplete`, `Calendar`, `Carousel`, `CarouselItem`, `CascaderPanel`, `Collapse`, `CollapseItem`, `ColorPicker`, `DatePicker`, `Descriptions`, `DescriptionsItem`, `DropdownItem`, `DropdownMenu`, `FormItem`, `Image`, `ImageViewer`, `InfiniteScroll`, `Loading`, `MessageBox`, `Option`, `OptionGroup`, `RadioButton`, `RadioGroup`, `Result`, `SelectV2`, `Spin`, `SubMenu`, `TableColumn`, `TimePicker`, `TimeSelect`, `Timeline`, `TimelineItem`, `TreeSelect`, `Upload`, `Watermark`, `SkeletonItem`

## CSS Strategy

The library uses **pure Rust CSS generation** exclusively — no CDN or external CSS dependencies.

Since 0.3.0, the style system is unified: `style_system.rs` is the single source of truth
for `Theme` (50 fields), `ThemeBuilder`, and `CompleteStyleManager`. Internally,
`CompleteStyleManager::generate_complete_styles()` delegates to
`styles::enhanced_css_system::all_styles()` (114 components).

```rust
use dioxus_element_plug::prelude::*;

// Generate complete Element Plus styles (114 components)
let css = CompleteStyleManager::new().generate_complete_styles();

rsx! {
style { "{css}" }
Button { variant: ButtonVariant::Primary, "Click me!" }
}
```

Per-component tree-shaking via `generate_styles_for_components()` is **deprecated** in
0.3.0 — it currently returns the complete stylesheet. It will return in 0.4.0.

Custom theme (via `ThemeBuilder`):

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

Or with struct update syntax (`Theme` has 50 fields since 0.3.0):

```rust
use dioxus_element_plug::{Theme, CompleteStyleManager};

let dark = Theme {
color_white: "#141414",
color_black: "#ffffff",
..Theme::default()
};
```

## Tests

- Tests use standard Rust `#[cfg(test)]` + `#[test]`
- Unit tests for enum `as_class()` methods and style constants
- Run with `cargo test --lib`
- Currently 5 tests, all passing

## Git & Review Hygiene

- Do not commit, push, or open pull requests unless explicitly asked
- Before committing: `cargo check` (zero errors/warnings) and `cargo test --lib` (all pass)
- Use conventional commit messages (e.g., `feat:`, `fix:`, `refactor:`)
- Repository is hosted on [Gitee](https://gitee.com/pauljoihn21/dioxus-element-plug)
