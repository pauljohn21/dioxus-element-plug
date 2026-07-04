//! # Dioxus Element Plug
//!
//! Element UI theme-chalk components for Dioxus applications.
//!
//! This crate provides a set of UI components styled with the popular Element UI theme-chalk CSS framework,
//! designed specifically for use with the Dioxus framework.
//!
//! ## Usage
//!
//! ## Modern DX: Dioxus 0.7+ Built-in SCSS
//!
//! Dioxus Element Plug uses manganis for automatic SCSS compilation:
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_element_plug::prelude::*;
//! use manganis::asset;
//!
//! static STYLES: Asset = asset!("/assets/theme-chalk.scss");
//!
//! fn App() -> Element {
//!     rsx! {
//!         Button {
//!             variant: ButtonVariant::Primary,
//!             "Click me!"
//!         }
//!     }
//! }
//! ```
//!

/// Re-export commonly used components
pub mod components;

/// CSS theme utilities and constants
pub mod theme;

/// SCSS asset utilities for Dioxus 0.7+ built-in SCSS support
/// Conditional compilation for docs.rs compatibility
#[cfg(not(docsrs))]
pub mod scss;

#[cfg(docsrs)]
pub mod scss {
    /// Minimal stub implementations for documentation generation
    pub mod prebuilt {}
    
    /// Re-export minimal manganis types for documentation
    pub use manganis;
    
    /// Asset macro stub for documentation
    pub use manganis::asset;
    
    /// Empty modules for documentation compatibility
    pub mod theme_variables {}
    pub mod typography {}
    pub mod spacing {}
    pub mod class_names {
        pub mod button {}
        pub mod input {}
        pub mod layout {}
    }
    pub mod helpers {}
}

/// Re-export manganis tools for convenient access
#[cfg(not(docsrs))]
pub use scss::{asset, class_names, helpers};

/// Re-export manganis for docs builds
#[cfg(docsrs)]
pub use manganis;

pub use components::*;
pub use theme::*;

/// Prelude module for easy importing
pub mod prelude {
    pub use crate::components::button::*;
    pub use crate::components::input::*;
    pub use crate::components::layout::*;
    pub use crate::theme::*;
    
    #[cfg(not(docsrs))]
    pub use crate::scss::{asset, class_names, helpers};
    
    #[cfg(docsrs)]
    pub use manganis;
}

// Simple test module to verify the library compiles
#[cfg(test)]
mod tests {
    

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
