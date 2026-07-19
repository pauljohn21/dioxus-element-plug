# Coding Standards - Dioxus Element Plug

## Rust Code Style

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Components | PascalCase | `Button`, `InputField` |
| Props structs | PascalCase + Props | `ButtonProps`, `InputProps` |
| Enums | PascalCase | `ButtonVariant`, `InputSize` |
| Enum variants | PascalCase | `Primary`, `Large` |
| Functions | snake_case | `as_class()`, `fire_event()` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_SIZE` |
| Files | snake_case | `button.rs`, `input_field.rs` |

### Component Structure

```rust
// 1. Imports
use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

// 2. Type/Variant enum (if applicable)
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl ButtonVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "el-button--primary",
            Self::Success => "el-button--success",
            Self::Warning => "el-button--warning",
            Self::Danger => "el-button--danger",
            Self::Info => "el-button--info",
        }
    }
}

// 3. Props struct
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    // Required
    pub children: Element,
    
    // With defaults
    #[props(default = ButtonVariant::Default)]
    pub variant: ButtonVariant,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    
    // Optional
    #[props(default)]
    pub size: Option<ButtonSize>,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

// 4. Component function
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Pre-extract values
    let class_string = ClassBuilder::new("el-button")
        .add_class(props.variant.as_class())
        .add_if("is-disabled", props.disabled)
        .add_if("is-loading", props.loading)
        .add_opt(props.class.as_ref())
        .build();
    
    let style_string = style_str(&props.style);
    let on_click = props.on_click;
    
    // Render
    rsx! {
        button {
            class: "{class_string}",
            style: "{style_string}",
            disabled: props.disabled,
            onclick: move |e| fire_event(&on_click, e),
            {props.children}
        }
    }
}
```

### Documentation Comments

```rust
/// Button component with variants and sizes.
/// 
/// # Example
/// ```rust
/// rsx! {
///     Button {
///         variant: ButtonVariant::Primary,
///         "Click me"
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // ...
}

/// Button size variants.
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Large,
    Medium,
    Small,
    Mini,
}

impl ButtonSize {
    /// Returns the CSS class for this size.
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Large => "el-button--large",
            Self::Medium => "",
            Self::Small => "el-button--small",
            Self::Mini => "el-button--mini",
        }
    }
}
```

### Error Handling

```rust
// Use Option for optional values
#[props(default)]
pub icon: Option<String>,

// Use fire_event for optional handlers
onclick: move |e| fire_event(&on_click, e),

// Handle None gracefully
let icon_class = props.icon.unwrap_or_default();
```

### Avoiding Clone

```rust
// ❌ BAD - clones in render loop
rsx! {
    for item in items.iter() {
        div { "{item.name.clone()}" }
    }
}

// ✅ GOOD - pre-collect owned values
let items: Vec<_> = items.iter().map(|i| i.name.clone()).collect();
rsx! {
    for name in items {
        div { "{name}" }
    }
}
```

## CSS Guidelines

### Class Naming

Follow Element Plus theme-chalk convention:

```rust
// Base component
el-button
el-input
el-table

// Modifiers (variants)
el-button--primary
el-button--large
el-input--disabled

// States
is-disabled
is-checked
is-focus
is-active

// Parts (child elements)
el-input__inner
el-input__wrapper
el-card__header
```

### ClassBuilder Usage

```rust
// Order: base → variant → state → custom
let class = ClassBuilder::new("el-component")
    .add_class(props.variant.as_class())     // variant first
    .add_class(props.size.as_class())        // then size
    .add_if("is-disabled", props.disabled)   // states
    .add_if("is-loading", props.loading)
    .add_opt(props.class.as_ref())           // user custom classes last
    .build();
```

## Testing Standards

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_as_class() {
        assert_eq!(ButtonVariant::Default.as_class(), "");
        assert_eq!(ButtonVariant::Primary.as_class(), "el-button--primary");
    }

    #[test]
    fn test_class_builder() {
        let class = ClassBuilder::new("el-test")
            .add_if("is-active", true)
            .add_if("is-disabled", false)
            .build();
        assert_eq!(class, "el-test is-active");
    }
}
```

### Test Commands

```bash
# Run all tests
cargo test --lib

# Run with output
cargo test --lib -- --nocapture

# Run specific test
cargo test --lib test_button_variant
```

## Git Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, semicolons, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Build process, dependencies, etc.

### Examples

```
feat: add Switch component with size variants

fix: correct Input focus state handling

docs: update README with dark mode examples

refactor: migrate Button to use ClassBuilder

test: add unit tests for ClassBuilder
```

## Code Review Checklist

Before submitting PR:

- [ ] `cargo check` passes with zero errors
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test --lib` passes all tests
- [ ] Component follows controlled pattern
- [ ] Uses ClassBuilder for class construction
- [ ] Pre-extracts data before `rsx!`
- [ ] Uses `fire_event` for optional handlers
- [ ] Includes `class` and `style` props
- [ ] Enum variants have `as_class()` method
- [ ] Documentation comments added
- [ ] Commit messages follow convention

## Performance Guidelines

1. **Avoid unnecessary clones** - Pre-collect iterator results
2. **Use `&'static str`** for CSS class strings
3. **Minimize signal subscriptions** - Extract values outside rsx!
4. **Lazy evaluation** - Use `use_memo` for expensive computations
5. **Conditional rendering** - Use `if` in rsx! instead of creating elements

```rust
// ❌ BAD
let expensive = compute_expensive(); // Always runs
rsx! {
    if show { "{expensive}" }
}

// ✅ GOOD
rsx! {
    if show {
        // Only compute when needed
        let expensive = use_memo(|| compute_expensive());
        "{expensive}"
    }
}
```
