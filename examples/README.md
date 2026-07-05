> ⚠️ This file has been auto-generated. Do not edit.

# Dioxus Element Plus - Examples

This directory contains example projects demonstrating how to use Dioxus Element Plus.

## Available Examples

### [`modern-pure-rust-example/`](./modern-pure-rust-example/)

**Recommended for beginners!**

A complete example showing the modern way to use Dioxus Element Plus with pure Rust styling.

#### What this example demonstrates:

- ✅ Global style injection using `CompleteStyleManager`
- ✅ All major components (Button, Input, Card, Grid, Alert)
- ✅ Type-safe component APIs
- ✅ Best practices and patterns
- ✅ Production-ready code

#### Quick start:

```bash
cd examples/modern-pure-rust-example
cargo run
# or
dx serve
```

#### Key features shown:

1. **Complete element plus compatibility** - All components styled and ready
2. **Zero CSS dependencies** - Pure Rust styling system
3. **Compile-time optimization** - Zero runtime CSS overhead
4. **Type safety** - Compile-time validation of all props

#### Running the example:

```bash
cd examples/modern-pure-rust-example
cargo run                   # Run directly
dx serve                    # Development server
dx serve --platform desktop # Desktop mode
dx build --release          # Production build
```

## Directory Structure

```
examples/
└── modern-pure-rust-example/
    ├── Cargo.toml          # Example configuration
    ├── Dioxus.toml         # Dioxus settings
    ├── README.md           # Detailed guide
    ├── input.css           # CSS placeholder (not used)
    ├── assets/             # Static assets
    └── src/main.rs         # Main example code
```

## Examples Overview

| Example | Key Features | Complexity | Use Case |
|---------|--------------|------------|----------|
| modern-pure-rust-example | All components, clean structure | Beginner | Learning & prototyping |

## Recommended Learning Path

1. **Start here**: modern-pure-rust-example
   - Learn the basics of pure Rust styling
   - Understand component APIs
   - See layout patterns in action

2. **Explore the main docs**: ../../README.md
   - Advanced theming
   - Component reference
   - Performance optimization

3. **Integrate into your projects**
   - Use the same patterns shown here
   - Customize themes for your brand
   - Build beautiful UIs with confidence

## Contributing

Want to add more examples? Great! Here's how:

1. Create a new directory under `examples/`
2. Follow the same structure as `modern-pure-rust-example/`
3. Include a comprehensive README.md
4. Make sure it compiles and runs
5. Submit a pull request

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

---

**Ready to build something amazing?**

Start with the [`modern-pure-rust-example`](./modern-pure-rust-example/) and create your next great UI! 🚀🦀