# Dioxus Element Plus - Examples

This directory contains example projects demonstrating how to use Dioxus Element Plus.

## Available Examples

### [`component-showcase/`](./component-showcase/)

A verification example covering 13 component categories, demonstrating the controlled component pattern.

#### What this example demonstrates:

- ✅ Global style injection using `CompleteStyleManager`
- ✅ 13 component categories: Button, Input, Select, Switch, Alert, Tag, Card, Dialog, Table, Tree, Cascader, Transfer, Progress
- ✅ Controlled component pattern (parent owns state via `use_signal`)
- ✅ Type-safe component APIs with `EventHandler` callbacks
- ✅ Production-ready code

#### Quick start:

```bash
cd examples/component-showcase
cargo check
dx serve
```

### [`theme-switcher/`](./theme-switcher/)

A theme switching demo showcasing 5 themes (Default, Dark, Green, Purple, Orange).

#### What this example demonstrates:

- ✅ Custom theme creation with `Theme::new()`
- ✅ Dynamic style generation with `CompleteStyleManager`
- ✅ Theme switching at runtime
- ✅ Multiple components under different themes

#### Quick start:

```bash
cd examples/theme-switcher
cargo check
dx serve
```

## Directory Structure

```
examples/
├── component-showcase/  # Component verification demo (13 categories)
└── theme-switcher/      # Theme switching demo (5 themes)
```

## Examples Overview

| Example | Key Features | Use Case |
|---------|--------------|----------|
| component-showcase | 13 component categories, controlled pattern | Component verification |
| theme-switcher | 5 themes, dynamic style generation | Theme customization |

## Recommended Learning Path

1. **Start here**: `component-showcase`
   - Learn the basics of pure Rust styling
   - Understand the controlled component pattern
   - See component APIs in action

2. **Explore theming**: `theme-switcher`
   - Learn how to create custom themes
   - Understand dynamic style generation
   - See theme switching at runtime

3. **Integrate into your projects**
   - Use the same patterns shown here
   - Customize themes for your brand
   - Build beautiful UIs with confidence

## Contributing

Want to add more examples? Great! Here's how:

1. Create a new directory under `examples/`
2. Follow the same structure as `component-showcase/`
3. Make sure it compiles with `cargo check`
4. Submit a pull request

## Troubleshooting

**Build errors?**
- Make sure you have the latest Rust toolchain
- Run `cargo clean && cargo build`
- Check the main project README for dependencies

**Styling not working?**
- Make sure you have the `style { "..." }` in your root component
- Verify you're using the correct component props
- Check the example for working patterns

**Questions?**
- Check the main project documentation: ../../README.md
- Look at the source code: ../../src/components/
- Review the theme system: ../../src/style_system.rs
