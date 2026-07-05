# Modern Pure Rust Example

A complete example demonstrating the modern way to use Dioxus Element Plus with pure Rust styling.

## 🎯 Features

- **Zero External CSS** - Pure Rust style generation
- **Complete API** - All major components demonstrated
- **Type Safety** - Compile-time validated components
- **Best Practices** - Production-ready patterns
- **Element Plus Compatibility** - Full theme support

## 🚀 Quick Start

### Running the Example

```bash
cd examples/modern-pure-rust-example

# Install dependencies (if needed)
cargo check

# Run the development server
dx serve

# Open in browser: http://localhost:8080
```

### Building for Production

```bash
cd examples/modern-pure-rust-example

# Build for production
dx build --release

# Serve the build
dx serve --release
```

## 📚 What You'll Learn

### 1. Global Style Injection

How to inject complete Element Plus styles into your app:

```rust
let styles = CompleteStyleManager::new().generate_complete_styles();

rsx! {
    style { "{styles}" }
    // Your app components here
}
```

### 2. Component Usage

All major components with practical examples:

- **Buttons**: Variants, sizes, states
- **Inputs**: Text, password, email inputs
- **Cards**: Content organization and layout
- **Alerts**: Success, warning, error, info messages
- **Grid**: Row/Col system with responsive design

### 3. Theming

Ready for theme customization:

```rust
// Custom theme example
let theme = Theme::default()
    .with_color_primary("#ff4081")
    .with_border_radius("8px");

let styles = CompleteStyleManager::new()
    .with_theme(theme)
    .generate_complete_styles();
```

### 4. Layout Patterns

Common layout patterns using the grid system:

```rust
// Full width
Row {
    Col { span: 24, "Full width content" }
}

// Three columns
Row {
    Col { span: 8, "Column 1" }
    Col { span: 8, "Column 2" }
    Col { span: 8, "Column 3" }
}

// Offset columns
Row {
    Col { span: 6, offset: 6, "Centered content" }
}
```

## 🔧 Technical Details

### Styling Approach

**Pure Rust CSS Generation:**
- Styles are generated at compile time
- Zero runtime CSS parsing
- No external dependencies (Node.js, Sass, etc.)
- Type-safe theme configuration

**Performance Benefits:**
- Immediate style application
- No Flash of Unstyled Content (FOUC)
- Tree-shaking friendly
- Minimal bundle size

### Component APIs

All components follow consistent patterns:

```rust
// Button with variant and click handler
Button {
    variant: ButtonVariant::Primary,
    on_click: move |_| handle_click(),
    "Click me!"
}

// Input with type and size
Input {
    placeholder: "Enter text",
    input_type: InputType::Text,
    size: InputSize::Medium,
}

// Alert with type
Alert {
    title: "Success!".to_string(),
    alert_type: AlertType::Success,
}
```

## 📖 Further Reading

- Main project [README](../../README.md) for complete documentation
- Component reference in [src/components/](../../src/components/)
- Theme system in [style_system.rs](../../src/style_system.rs)

## 🎨 Customization

### Custom Colors

```rust
use dioxus_element_plug::style_system::Theme;

let custom_theme = Theme::default();
custom_theme.color_primary = "#ff6b6b";
custom_theme.color_success = "#40a9ff";
```

### Component-Specific Styles

```rust
// Only generate styles for used components
let styles = CompleteStyleManager::new()
    .generate_styles_for_components(&["button", "input", "alert"]);
```

## 🚀 Next Steps

1. **Run the example** and explore the code
2. **Modify the theme** to see live updates
3. **Add your own components** using the same patterns
4. **Check the main project docs** for advanced features
5. **Integrate into your projects** with confidence

This example demonstrates the future of Dioxus UI development: beautiful, type-safe, and completely in Rust! 🦀✨