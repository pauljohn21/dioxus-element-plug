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
//!     // Generate complete Element Plus styles (114 components)
//!     let styles = CompleteStyleManager::new()
//!         .generate_complete_styles();
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
/// This module provides comprehensive styling without SCSS dependencies.
///
/// Since 0.3.0, this is the single source of truth for the theme system:
/// `Theme` (50 fields), `ThemeBuilder`, `CompleteStyleManager`, and the
/// deprecated `CompleteCssBuilder` all live here.
#[allow(deprecated)]
pub use style_system::{Theme, ThemeBuilder, CompleteStyleManager, CompleteCssBuilder, StyleManager, ALL_COMPONENTS};

/// Theme and style re-exports from modular system
pub use crate::styles::prelude::*;

pub use components::*;

/// Prelude module for easy importing
pub mod prelude {
    // Core components
    pub use crate::components::button::*;
    pub use crate::components::input::*;
    pub use crate::components::input_number::*;
    pub use crate::components::layout::*;
    pub use crate::components::form::*;
    pub use crate::components::form_item::*;
    pub use crate::components::alert::*;
    pub use crate::components::card::*;
    pub use crate::components::table::*;
    pub use crate::components::table_column::*;

    // Basic UI
    pub use crate::components::tag::*;
    pub use crate::components::badge::*;
    pub use crate::components::link::*;
    pub use crate::components::divider::*;
    pub use crate::components::empty::*;
    pub use crate::components::avatar::*;
    pub use crate::components::avatar_group::*;
    pub use crate::components::button_group::*;
    pub use crate::components::icon::*;

    // Form
    pub use crate::components::switch::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::radio::*;
    pub use crate::components::select::*;
    pub use crate::components::option::*;
    pub use crate::components::slider::*;
    pub use crate::components::rate::*;
    pub use crate::components::autocomplete::*;
    pub use crate::components::cascader::*;
    pub use crate::components::color_picker::*;
    pub use crate::components::date_picker::*;
    pub use crate::components::time_picker::*;
    pub use crate::components::time_select::*;
    pub use crate::components::transfer::*;

    // Display
    pub use crate::components::dialog::*;
    pub use crate::components::drawer::*;
    pub use crate::components::collapse::*;
    pub use crate::components::result::*;
    pub use crate::components::image::*;
    pub use crate::components::image_viewer::*;
    pub use crate::components::descriptions::*;
    pub use crate::components::calendar::*;
    pub use crate::components::carousel::*;
    pub use crate::components::timeline::*;
    pub use crate::components::tree::*;
    pub use crate::components::upload::*;
    pub use crate::components::watermark::*;
    pub use crate::components::progress::*;
    pub use crate::components::skeleton::*;
    pub use crate::components::space::*;

    // Navigation
    pub use crate::components::breadcrumb::*;
    pub use crate::components::dropdown::*;
    pub use crate::components::menu::*;
    pub use crate::components::pagination::*;
    pub use crate::components::tabs::*;
    pub use crate::components::steps::*;
    pub use crate::components::page_header::*;
    pub use crate::components::affix::*;
    pub use crate::components::anchor::*;
    pub use crate::components::backtop::*;

    // Feedback
    pub use crate::components::loading::*;
    pub use crate::components::message::*;
    pub use crate::components::message_box::*;
    pub use crate::components::notification::*;
    pub use crate::components::tooltip::*;
    pub use crate::components::popover::*;
    pub use crate::components::popconfirm::*;
    pub use crate::components::spin::*;
    pub use crate::components::infinite_scroll::*;

    // Style system
    pub use crate::Theme;
    pub use crate::ThemeBuilder;
    pub use crate::CompleteStyleManager;
    pub use crate::style_system::StyleManager;
    pub use crate::style_system::ALL_COMPONENTS;
    #[allow(deprecated)]
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
