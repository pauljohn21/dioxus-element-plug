# Quick Start Guide

Get up and running with Dioxus Theme Chalk in minutes.

## Setup

### 1. Install Prerequisites

You'll need:
- Rust (latest stable)
- Node.js 16+
- npm or yarn

#### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

#### Install Node.js
Download from [nodejs.org](https://nodejs.org/) or use your package manager.

### 2. Clone and Setup

```bash
# Navigate to your project
cd /Users/pauljohn/rust/theme-chalk

# Install development dependencies
make setup
```

### 3. Build the CSS

**选择你的 CSS 处理方式：**

#### 🚀 方式一：Dioxus 0.7 内置 SCSS 支持 (推荐)
使用 manganis asset 宏，无需单独的构建步骤：

```rust
// 在你的组件文件中
use manganis::*;
static STYLES: Asset = asset!("/assets/theme-chalk.scss");
```

查看 `examples/with-scss-asset/` 目录获取完整示例。

#### 🔧 方式二：传统外部编译方式
```bash
# Compile SCSS to CSS using external Sass compiler
make build-css

# Or watch for changes during development
make watch-css
```

### 4. Build the Rust Library

```bash
# Development build
cargo build

# Release build
cargo build --release
```

## Integration with Your Dioxus App

### 1. Add to Cargo.toml

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-element-plug = { path = "../dioxus-element-plug" }
```

### 2. Copy CSS to Your Assets

```bash
cp dist/theme-chalk.css /path/to/your/project/assets/
```

### 3. Add CSS to HTML

In your `index.html`:
```html
<link rel="stylesheet" href="/assets/theme-chalk.css">
```

### 4. Use Components

```rust
use dioxus::prelude::*;
use dioxus_theme_chalk::prelude::*;

fn MyComponent() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            "Hello World!"
        }
    }
}
```

## Running the Example

```bash
# Build CSS first
make build-css

# Run the example
cd examples/basic
trunk serve
```

Visit `http://localhost:8080` to see the example.

## Development Workflow

### Terminal 1: Watch CSS Changes
```bash
cd /Users/pauljohn/rust/theme-chalk
make watch-css
```

### Terminal 2: Run Your App
```bash
cd /path/to/your/dioxus/app
trunk serve
```

### Terminal 3: Rust Development
```bash
cd /Users/pauljohn/rust/theme-chalk
cargo check
# or
cargo watch -x check
```

## Common Tasks

### Update Dependencies
```bash
cargo update
npm update  # if using package.json
```

### Clean Build
```bash
make clean
cargo clean
```

### Format Code
```bash
cargo fmt
```

### Run Tests
```bash
cargo test
```

### Check for Issues
```bash
cargo clippy
```

## Folder Structure

```
/Users/pauljohn/rust/theme-chalk/
├── Cargo.toml          # Rust project config
├── Makefile           # Build automation
├── src/               # Rust source code
│   ├── lib.rs         # Library entry point
│   ├── theme.rs       # Theme constants
│   └── components/    # UI components
│       ├── mod.rs     # Component exports
│       ├── button.rs  # Button component
│       ├── input.rs   # Input component
│       └── layout.rs  # Layout components
├── build.rs           # Build script
├── dist/              # Compiled CSS (generated)
├── examples/          # Example applications
└── ../src/            # Original SCSS theme files
```

## Troubleshooting

### "sass command not found"
```bash
npm install -g sass
```

### CSS not loading in browser
- Make sure the CSS file path in your HTML matches your server setup
- Check browser console for 404 errors
- Ensure `make build-css` completed successfully

### Component styles not applying
- Verify theme-chalk.css is loaded before your app
- Check that you're using the correct component variants
- Inspect elements in browser dev tools

### Build errors
- Ensure you're using Rust stable: `rustup default stable`
- Update dependencies: `cargo update`
- Clear and rebuild: `cargo clean && cargo build`

## Next Steps

1. **Explore Components**: Check the [README.md](README.md) for detailed component documentation
2. **Customize Theme**: Override CSS variables to match your brand
3. **Add Components**: Contribute new components following the existing patterns
4. **Performance**: Use `cargo build --release` for production builds

## Getting Help

- Check the [README.md](README.md) for detailed documentation
- Look at the [examples/](examples/) directory for usage examples
- Open an issue on the GitHub repository
- Check the [CHANGELOG.md](CHANGELOG.md) for updates
