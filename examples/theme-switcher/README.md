# Theme Switcher Example

Demonstrates runtime theme switching with pure Rust CSS generation.

## Features

- 5 built-in themes: Default Blue, Dark Mode, Green Nature, Purple Dream, Orange Sunset
- Instant theme switching via `CompleteStyleManager::with_theme()`
- All CSS generated in pure Rust - zero external dependencies
- Theme struct allows customizing colors, border radius, fonts, and more

## Running

```bash
cd examples/theme-switcher
dx serve
```

## How It Works

1. Define a `Theme` struct with custom colors and design tokens
2. Pass the theme to `CompleteStyleManager::new().with_theme(theme)`
3. Call `generate_complete_styles()` to get the CSS string
4. Inject the CSS into a `<style>` tag in your RSX
5. On theme switch, regenerate the CSS - Dioxus re-renders automatically
