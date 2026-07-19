# CLAUDE.md - Claude Code Instructions

## Project Overview

**dioxus-element-plug** (v0.3.0) is a Rust UI component library for Dioxus 0.7, implementing Element Plus design system with pure Rust CSS generation.

## Repository Structure

```
dioxus-element-plug/
├── src/components/          # 97 component files
│   ├── common.rs           # ClassBuilder, style_str, fire_event
│   ├── mod.rs              # Module declarations
│   └── *.rs                # Individual components
├── element-icons/          # 137+ SVG icons (optional feature)
├── src/style_system.rs     # Theme, CompleteStyleManager
├── src/styles/             # CSS constants
├── examples/               # Showcase and theme-switcher
├── .cursorrules            # Cursor IDE rules
├── AGENTS.md               # Detailed agent guidance
└── SKILL.md                # Skill documentation
```

## Core Principles

### 1. Controlled Components Only

State is ALWAYS owned by parent, never by components:

```rust
// Parent
let mut value = use_signal(|| false);

// Component usage
rsx! {
    Switch {
        model_value: value(),
        on_change: move |v| value.set(v),
    }
}
```

### 2. ClassBuilder Pattern

Always use ClassBuilder for CSS class construction:

```rust
let class = ClassBuilder::new("el-button")
    .add_class(ButtonVariant::Primary.as_class())
    .add_if("is-disabled", props.disabled)
    .add_if("is-loading", props.loading)
    .add_opt(props.class.as_ref())
    .build();
```

### 3. Pre-extract Pattern

Extract values before `rsx!` to avoid ownership issues:

```rust
// ✅ GOOD
let options: Vec<_> = props.options.iter().map(|o| (o.value.clone(), o.label.clone())).collect();
let on_change = props.on_change;

rsx! {
    for (value, label) in options {
        div {
            onclick: move |_| fire_event(&on_change, value.clone()),
            "{label}"
        }
    }
}

// ❌ BAD - borrowing props in closure
rsx! {
    for option in props.options.iter() {
        div {
            onclick: move |_| on_change.call(option.value.clone()), // ERROR
        }
    }
}
```

## Component Implementation Checklist

When creating/modifying components:

- [ ] Use `#[derive(Props, Clone, PartialEq)]` for props
- [ ] Include `children: Element` (unless optional)
- [ ] Include `class: Option<String>` and `style: Option<String>`
- [ ] Use `#[props(default = false)]` for booleans
- [ ] Use `#[props(default)]` for Option fields
- [ ] Implement `as_class()` for all enum variants
- [ ] Use ClassBuilder for class construction
- [ ] Use `style_str()` for style extraction
- [ ] Use `fire_event()` for optional handlers
- [ ] Pre-extract data before `rsx!`

## Common Utilities

### ClassBuilder

```rust
use crate::components::common::ClassBuilder;

ClassBuilder::new("el-component")
    .add_class("base-class")           // unconditional
    .add_if("is-state", condition)     // conditional
    .add_opt(Some("extra"))            // Option<&String>
    .add_opt_str(Some("str"))          // Option<&str>
    .build()
```

### style_str

```rust
use crate::components::common::style_str;
let style = style_str(&props.style);  // String, empty if None
```

### fire_event

```rust
use crate::components::common::fire_event;

// In rsx!:
onclick: move |e| fire_event(&on_click, e),
```

## CSS Class Reference

| Pattern | Example | Usage |
|---------|---------|-------|
| Base | `el-button` | Component root |
| Modifier | `el-button--primary` | Variant/type |
| State | `is-disabled` | Boolean state |
| Part | `el-input__inner` | Child element |

## Event Handlers Reference

| Component | Prop | Type |
|-----------|------|------|
| Button | `on_click` | `EventHandler<MouseEvent>` |
| Input | `on_input` | `EventHandler<FormEvent>` |
| Input | `on_focus` | `EventHandler<FocusEvent>` |
| Select | `on_change` | `EventHandler<String>` |
| Switch | `on_change` | `EventHandler<bool>` |
| Checkbox | `on_change` | `EventHandler<bool>` |
| Rate | `on_change` | `EventHandler<u32>` |
| Slider | `on_change` | `EventHandler<u32>` |
| InputNumber | `on_change` | `EventHandler<i64>` |
| Dialog | `on_close` | `EventHandler<()>` |
| Pagination | `on_current_change` | `EventHandler<u32>` |

## Multi-Component Files

Some files define multiple components:

- `container.rs` → Container, Header, Footer, Main, Aside
- `row.rs` → Row, Col
- `menu.rs` → Menu, MenuItem, SubMenu, MenuItemGroup
- `steps.rs` → Steps, Step
- `dropdown.rs` → Dropdown, DropdownMenu, DropdownItem
- `checkbox.rs` → Checkbox, CheckboxGroup, CheckboxButton
- `radio.rs` → Radio, RadioGroup, RadioButton
- `button.rs` → Button, OutlineButton, TextButton, LinkButton

## Icons (v0.3.0+)

```rust
// With icons feature (default enabled)
use element_icons::element::{ArrowLeft, ArrowRight, Search, Close};

rsx! {
    Button {
        ArrowLeft {}
        "Back"
    }
}
```

Feature flag in Cargo.toml:
```toml
[features]
default = ["icons"]
icons = ["dep:element-icons"]
```

## Dark Mode (v0.3.0+)

```rust
use dioxus_element_plug::{Theme, CompleteStyleManager};

// Built-in presets
let dark = Theme::dark();
let light = Theme::light();

// Custom theme
let custom = Theme {
    primary_color: "#1890ff".to_string(),
    ..Theme::default()
};

let styles = CompleteStyleManager::new()
    .with_theme(dark)
    .generate_complete_styles();
```

## Testing

```bash
# Verify compilation
cargo check

# Run tests
cargo test --lib

# Lint check
cargo clippy

# Check examples
cd examples/component-showcase && cargo check
cd examples/theme-switcher && cargo check
```

## Documentation

- Update AGENTS.md for detailed architecture guidance
- Update SKILL.md for high-level skill documentation
- Update README.md for user-facing documentation
- Keep .cursorrules in sync with code changes

## Git Workflow

1. Before committing: `cargo check` (zero errors)
2. Run tests: `cargo test --lib` (all pass)
3. Use conventional commits: `feat:`, `fix:`, `refactor:`
4. Repository: https://gitee.com/pauljoihn21/dioxus-element-plug
