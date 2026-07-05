//! # Dioxus Element Plug
//!
//! Element UI components for Dioxus applications with pure Rust styling system.
//!
//! This crate provides a set of UI components styled with the popular Element UI theme-chalk design,
//! designed specifically for use with the Dioxus framework. All styling is generated in pure Rust
//! for zero runtime overhead and maximum performance.
//!
//! ## Features
//!
//! - 🎨 **Pure Rust styling** - Zero runtime overhead with compile-time CSS generation
//! - 🦀 **Rust-native components** - Type-safe components built for Dioxus
//! - 📦 **Ready to use** - Components work out of the box with proper styling
//! - 🎯 **Consistent API** - Intuitive props and events matching Dioxus patterns
//! - 📱 **Responsive design** - Mobile-friendly components with flexible grid system
//! - 🔥 **Tree-shaking friendly** - Only include styles for components you use
//! - ⚡ **Zero dependencies** - No SCSS compilation required at runtime
//!
//! ## Usage
//!
//! ### Basic Usage with Pure Rust Styling
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_element_plug::prelude::*;
//! use dioxus_element_plug::style_system::CompleteStyleManager;
//!
//! fn App() -> Element {
//!     // Generate styles for specific components
//!     let styles = CompleteStyleManager::new()
//!         .generate_styles_for_components(&["button", "input", "alert"]);
//!
//!     rsx! {
//!         style { "{styles}" }
//!         
//!         div {
//!             Button {
//!                 variant: ButtonVariant::Primary,
//!                 "Click me!"
//!             }
//!             
//!             Input {
//!                 placeholder: "Enter text...",
//!                 size: Some(InputSize::Medium),
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ### Generate Complete Stylesheet
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_element_plug::prelude::*;
//!
//! fn App() -> Element {
//!     // Generate complete Element Plus styles
//!     let complete_styles = CompleteStyleManager::new()
//!         .generate_complete_styles();
//!
//!     rsx! {
//!         style { "{complete_styles}" }
//!         
//!         // Use any Element Plus component
//!         Button {
//!             variant: ButtonVariant::Primary,
//!             "Primary Button"
//!         }
//!         
//!         Alert {
//!             title: "Success!".to_string(),
//!             alert_type: AlertType::Success,
//!         }
//!     }
//! }
//! ```

/// Re-export commonly used components
pub mod components;

/// Pure Rust CSS generation system - zero runtime overhead styling solution
pub mod style_system;

/// Modular Element Plus style system (SCSS to Rust converted)
pub mod styles;

/// Pure Rust styling system
/// This module provides comprehensive styling without SCSS dependencies
pub use style_system::{Theme, CompleteStyleManager, CompleteCssBuilder};

/// Theme and style re-exports from modular system
pub use crate::styles::prelude::*;

pub use components::*;

/// Prelude module for easy importing
pub mod prelude {
    pub use crate::components::button::*;
    pub use crate::components::input::*;
    pub use crate::components::layout::*;
    pub use crate::components::form::*;
    pub use crate::components::alert::*;
    pub use crate::components::card::*;
    pub use crate::components::table::*;
    pub use crate::Theme;
    pub use crate::CompleteStyleManager;
    pub use crate::CompleteCssBuilder;
    
    /// Complete Element Plus styling from modular system
    pub use crate::styles::prelude::*;
}

// Simple test module to verify the library compiles
#[cfg(test)]
mod tests {
    

    #[test]
    fn test_generated_style_constants() {
        use crate::styles::prelude::*;
        assert_eq!(COLOR_PRIMARY, "#409EFF");
        assert_eq!(COLOR_SUCCESS, "#67C23A");
        assert_eq!(COLOR_WARNING, "#E6A23C");
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
