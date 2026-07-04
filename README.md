# Dioxus Element Plug

<a href="https://github.com/pauljohn21/dioxus-element-plug">
  <img src="https://img.shields.io/github/stars/pauljohn21/dioxus-element-plug?style=social" alt="GitHub stars">
  <img src="https://img.shields.io/github/forks/pauljohn21/dioxus-element-plug?style=social" alt="GitHub forks">
  <img src="https://img.shields.io/github/issues/pauljohn21/dioxus-element-plug" alt="GitHub issues">
  <img src="https://img.shields.io/github/license/pauljohn21/dioxus-element-plug" alt="GitHub license">
</a>

Element UI theme-chalk components for Dioxus applications.

This crate provides a set of UI components styled with the popular Element UI theme-chalk CSS framework, designed specifically for use with the Dioxus framework. **Hosted on GitHub: [pauljohn21/dioxus-element-plug](https://github.com/pauljohn21/dioxus-element-plug)**

## Features

- 🎨 **Complete Element UI styling** - All components use authentic theme-chalk CSS
- 🦀 **Rust-native components** - Type-safe components built for Dioxus 0.7
- 📦 **Ready to use** - Components work out of the box with proper styling
- 🎯 **Consistent API** - Intuitive props and events matching Dioxus patterns
- 📱 **Responsive design** - Mobile-friendly components with flexible grid system
- 🔧 **Active Development** - Regular updates and community contributions

## Quick Start

### 1. Add to your Cargo.toml

For local development:
```toml
[dependencies]
dioxus-element-plug = { path = "../dioxus-element-plug" }
```

For production use (once published to crates.io):
```toml
[dependencies]
dioxus-element-plug = "0.1.0"
```

Or use directly from GitHub:
```toml
dioxus-element-plug = { git = "https://github.com/pauljohn21/dioxus-element-plug.git" }
```

### 2. Build the CSS

```bash
cd dioxus-element-plug
make setup    # Install dependencies
make build-css # Compile CSS
```

### 3. Add CSS to your HTML

Add this to your `index.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="/dist/theme-chalk.css">
</head>
<body>
    <div id="main"></div>
</body>
</html>
```

### 4. Use the components

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    rsx! {
        Container {
            Header {
                h1 { "My App" }
            }
            Main {
                Row {
                    Col { span: Some(12) {
                        h2 { "Welcome!" }
                        Button {
                            variant: ButtonVariant::Primary,
                            "Click me!",
                        }
                        Input {
                            placeholder: "Enter text...",
                            size: Some(InputSize::Medium),
                        }
                    }}
                }
            }
        }
    }
}
```

## Available Components

### Layout
- `Container` - Main container wrapper
- `Header` - Page header
- `Aside` - Sidebar
- `Main` - Main content area
- `Footer` - Page footer
- `Row`/`Col` - Grid system

### Form Controls
- `Button` - Various button styles and sizes
- `Input` - Text inputs with validation
- `SearchInput` - Search input with icon
- `PasswordInput` - Password input with toggle

### Soon to come
- Forms (Checkbox, Radio, Select)
- Navigation (Menu, Tabs, Breadcrumb)
- Feedback (Alert, Message, Notification)
- Data Display (Table, Card, List)

## Component Examples

### Buttons

```rust
use dioxus_element_plug::components::button::*;

rsx! {
    div {
        Button { variant: ButtonVariant::Primary, "Primary" }
        Button { variant: ButtonVariant::Success, "Success" }
        Button { variant: ButtonVariant::Warning, "Warning" }
        Button { variant: ButtonVariant::Danger, "Danger" }
        OutlineButton { variant: ButtonVariant::Info, "Info Outline" }
        TextButton { "Text Button" }
    }
}
```

### Input

```rust
use dioxus_element_plug::components::input::*;

rsx! {
    div {
        Input {
            placeholder: "Enter your name",
            size: Some(InputSize::Large),
            on_input: move |evt| println!("Input: {}", evt.value()),
        }
        PasswordInput {
            placeholder: "Password",
            clearable: true,
        }
        SearchInput {
            placeholder: "Search...",
            prefix_icon: Some("el-icon-search".to_string()),
        }
    }
}
```

### Layout Grid

```rust
use dioxus_element_plug::components::layout::*;

rsx! {
    Container {
        Header { height: 60 {
            h1 { "My App" }
        }}
        Main {
            Row { gutter: Some(20) {
                Col { span: Some(8) {
                    p { "Column 1" }
                }}
                Col { span: Some(8) {
                    p { "Column 2" }
                }}
                Col { span: Some(8) {
                    p { "Column 3" }
                }}
            }}
        }
        Footer { height: 60 {
            p { "Footer" }
        }}
    }
}
```

## Building CSS

The theme CSS needs to be compiled from SCSS:

### Prerequisites
- Node.js 16+
- npm or yarn

### Setup and Build

```bash
# Setup (installs sass compiler)
make setup

# Build CSS
make build-css

# Watch for changes during development
make watch-css

# Full build (Rust + CSS)
make build
```

## Development

### Project Structure

```
dioxus-theme-chalk/
├── src/               # Rust component library
│   ├── components/    # UI components
│   └── theme.rs       # Theme constants
├── build.rs           # Build script
├── Cargo.toml         # Rust dependencies
├── Makefile          # Build automation
├── README.md         # Documentation
└── example/          # Example app (coming soon)
```

### Working with SCSS

The original SCSS files are in the parent `src/` directory. The Makefile compiles them to the `dist/` directory.

To watch for changes while developing:

```bash
# Terminal 1: Watch Rust code
cargo watch -x run

# Terminal 2: Watch SCSS changes
make watch-css
```

## Theming

This library uses the exact same CSS classes and variables as Element UI. You can customize the theme by:

1. **Override CSS variables** in your custom CSS
2. **Modify the SCSS source** before building
3. **Use custom classes** via the `class` prop on components

### CSS Variables

Key variables you can override:

```css
:root {
    --el-color-primary: #409eff;
    --el-font-size-base: 14px;
    --el-border-radius-base: 4px;
    /* ... */
}
```

## License

MIT License - see LICENSE file for details.

## Contributing

We welcome contributions from the community! Please feel free to submit issues and pull requests on our [GitHub repository](https://github.com/pauljohn21/dioxus-element-plug).

### Development Setup

1. **Fork the repository** on [GitHub](https://github.com/pauljohn21/dioxus-element-plug/fork)
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/dioxus-element-plug.git
   cd dioxus-element-plug
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes** and test locally
5. **Commit with a clear message**:
   ```bash
   git commit -m "feat: add new awesome feature"
   ```
6. **Push and create a Pull Request** on GitHub

### GitHub Workflow
- **Issues**: Report bugs and feature requests [here](https://github.com/pauljohn21/dioxus-element-plug/issues)
- **Pull Requests**: Submit improvements [here](https://github.com/pauljohn21/dioxus-element-plug/pulls)
- **Discussions**: Join conversations [here](https://github.com/pauljohn21/dioxus-element-plug/discussions)

We use conventional commit messages and semantic versioning. Please ensure your code follows the existing style and includes appropriate tests.

## Credits

- [Element UI](https://element.eleme.io/) - Original design system and theme-chalk CSS
- [Dioxus](https://dioxuslabs.com/) - React-like framework for Rust
- [GitHub Contributors](https://github.com/pauljohn21/dioxus-element-plug/graphs/contributors) - Open source community

## License

MIT License - see LICENSE file for details.

## Star History

If you find this project helpful, please consider giving it a ⭐ on [GitHub](https://github.com/pauljohn21/dioxus-element-plug)!

---

**Ready to level up your Dioxus apps with beautiful Element UI styling? Start building today!** 🚀

---

### Project Status

This project is actively maintained on GitHub. Check out our:
- 📈 [Project Insights](https://github.com/pauljohn21/dioxus-element-plug/pulse)
- 🏗️ [Automated Builds](https://github.com/pauljohn21/dioxus-element-plug/actions)
- 📊 [Code Quality](https://github.com/pauljohn21/dioxus-element-plug/graphs/contributors)
- 🔄 [Release Notes](https://github.com/pauljohn21/dioxus-element-plug/releases)

### Join the Community

- 💬 [GitHub Discussions](https://github.com/pauljohn21/dioxus-element-plug/discussions)
- 🐛 [Issue Tracker](https://github.com/pauljohn21/dioxus-element-plug/issues)
- 🌟 [Star the Repository](https://github.com/pauljohn21/dioxus-element-plug)
- 🍴 [Fork for Your Projects](https://github.com/pauljohn21/dioxus-element-plug/fork)

**Together, we're building the future of Dioxus UI development!** 🎉
