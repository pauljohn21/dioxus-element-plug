//! # Dioxus Element Plug
//!
//! Element UI theme-chalk components for Dioxus applications.
//!
//! This crate provides a set of UI components styled with the popular Element UI theme-chalk CSS framework,
//! designed specifically for use with the Dioxus framework.
//!
//! ## Usage
//!
//! First, add the theme CSS to your HTML:
//!
//! ```html
//! <link rel="stylesheet" href="/assets/theme-chalk.css">
//! ```
//!
//! Then use the components in your Dioxus app:
//!
//! ```rust,ignore
//! use dioxus_theme_chalk::components::button::Button;
//!
//! fn App() -> Element {
//!     rsx! {
//!         Button { variant: "primary", "Click me!" }
//!     }
//! }
//! ```

/// Re-export commonly used components
pub mod components;

/// CSS theme utilities and constants
pub mod theme;

pub use components::*;
pub use theme::*;

/// Prelude module for easy importing
pub mod prelude {
    pub use crate::components::button::*;
    pub use crate::components::input::*;
    pub use crate::components::layout::*;
    pub use crate::theme::*;
}

// Simple test module to verify the library compiles
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_constants() {
        use crate::theme::colors;
        assert_eq!(colors::PRIMARY, "#409EFF");
        assert_eq!(colors::SUCCESS, "#67C23A");
        assert_eq!(colors::WARNING, "#E6A23C");
    }

    #[test]
    fn test_button_variants() {
        use crate::components::button::ButtonVariant;

        let primary = ButtonVariant::Primary;
        assert_eq!(primary.as_class(), "el-button--primary");

        let default = ButtonVariant::Default;
        assert_eq!(default.as_class(), "el-button");
    }

    #[test]
    fn test_input_variants() {
        use crate::components::input::InputType;

        let text = InputType::Text;
        assert_eq!(text.as_str(), "text");

        let password = InputType::Password;
        assert_eq!(password.as_str(), "password");
    }
}
