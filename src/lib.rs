//! # Dioxus Element Plug
//!
//! Element Plus components for Dioxus applications with pure Rust styling system.
//!
//! This crate provides 115+ UI components styled with Element Plus theme-chalk design,
//! designed specifically for use with the Dioxus 0.7 framework.
//!
//! ## Features
//!
//! - 🎨 **115+ Element Plus components** - Complete coverage of all component categories
//! - 🦀 **Pure Rust styling** - Zero runtime overhead with compile-time CSS generation
//! - 🎯 **Controlled component pattern** - State managed by parent via props + events
//! - 📦 **Ready to use** - Components work out of the box with Element Plus CSS
//! - ⚡ **Zero SCSS dependencies** - Pure Rust CSS generation system
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_element_plug::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         document::Link {
//!             rel: "stylesheet",
//!             href: "//unpkg.com/element-plus@2.4.4/dist/index.css"
//!         }
//!
//!         Button {
//!             variant: ButtonVariant::Primary,
//!             on_click: move |_: MouseEvent| println!("Clicked!"),
//!             "Click me!"
//!         }
//!
//!         Input {
//!             value: "Hello".to_string(),
//!             on_change: move |e: Event<FormData>| println!("{}", e.value()),
//!         }
//!     }
//! }
//! ```

/// Re-export all components
pub mod components;

/// Pure Rust CSS generation system
pub mod style_system;

/// Modular Element Plus style system
pub mod styles;

/// Styling system re-exports
pub use style_system::{Theme, CompleteStyleManager, CompleteCssBuilder};
pub use crate::styles::prelude::*;
pub use components::*;

/// Prelude module for easy importing of all components and types
#[allow(ambiguous_glob_reexports)]
#[allow(unused_imports)]
pub mod prelude {
    // Core components
    pub use crate::components::button::*;
    pub use crate::components::input::*;
    pub use crate::components::input_number::*;
    pub use crate::components::select::*;
    pub use crate::components::switch::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::radio::*;
    pub use crate::components::slider::*;
    pub use crate::components::rate::*;

    // Layout
    // layout.rs removed - use container.rs and row.rs instead
    pub use crate::components::container::*;
    pub use crate::components::row::*;
    pub use crate::components::col::*;
    pub use crate::components::space::*;

    // Display
    pub use crate::components::card::*;
    pub use crate::components::alert::*;
    pub use crate::components::tag::*;
    pub use crate::components::progress::*;
    pub use crate::components::badge::*;
    pub use crate::components::empty::*;
    pub use crate::components::divider::*;
    pub use crate::components::link::*;
    pub use crate::components::skeleton::*;
    pub use crate::components::table::*;
    pub use crate::components::tree::*;

    // Navigation
    pub use crate::components::menu::*;
    pub use crate::components::tabs::*;
    pub use crate::components::steps::*;
    pub use crate::components::pagination::*;
    pub use crate::components::breadcrumb::*;
    pub use crate::components::dropdown::*;
    pub use crate::components::affix::*;
    pub use crate::components::page_header::*;

    // Feedback
    pub use crate::components::dialog::*;
    pub use crate::components::drawer::*;
    pub use crate::components::tooltip::*;
    pub use crate::components::popover::*;
    pub use crate::components::popconfirm::*;
    pub use crate::components::message::*;
    pub use crate::components::notification::*;

    // Form
    pub use crate::components::form::*;
    pub use crate::components::cascader::*;
    pub use crate::components::transfer::*;

    // Icons
    pub use crate::components::icon::*;

    // Styling
    pub use crate::Theme;
    pub use crate::CompleteStyleManager;
    pub use crate::CompleteCssBuilder;
    pub use crate::styles::prelude::*;
}

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

    #[test]
    fn test_tag_types() {
        use crate::components::tag::TagType;

        let primary = TagType::Primary;
        assert_eq!(primary.as_class(), "el-tag--primary");

        let danger = TagType::Danger;
        assert_eq!(danger.as_class(), "el-tag--danger");
    }

    #[test]
    fn test_badge_types() {
        use crate::components::badge::BadgeType;

        let primary = BadgeType::Primary;
        assert_eq!(primary.as_class(), "el-badge__content--primary");
    }

    #[test]
    fn test_switch_size() {
        use crate::components::switch::SwitchSize;

        let large = SwitchSize::Large;
        assert_eq!(large.as_class(), "el-switch--large");
    }
}
