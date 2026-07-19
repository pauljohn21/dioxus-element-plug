//! # Styles Module - Modular Element Plus Style System
//!
//! This module organizes the complete Element Plus style system into
//! modular, maintainable files instead of a single large generated_styles.rs.
//!
//! The system includes:
//! - CSS class constants for all components
//! - Component CSS generation functions
//! - Theme system and variables
//! - Utility classes and helpers

// Core style constants and system
pub mod core;
pub mod colors;
pub mod typography;
pub mod spacing;
pub mod borders;
pub mod shadows;
pub mod z_index;

// Component style constants
pub mod component_classes;

// CSS generation functions
pub mod simple_css_generation;
pub mod enhanced_css_system;

// Theme system
pub mod theme;

// Utility classes
pub mod utilities;

// Enhanced output and complete CSS generation
pub mod enhanced_output;

// Pre-built CSS snippets
pub mod css_snippets;

pub mod prelude {
    //! Prelude module for easy importing of all style constants and functions
    //!
    //! Prelude for complete Element Plus style system
    //!
    //! Import everything with: `use dioxus_element_plug::styles::prelude::*;`

    // Core system
    pub use super::core::*;
    pub use super::colors::*;
    pub use super::typography::*;
    pub use super::spacing::*;
    pub use super::borders::*;
    pub use super::shadows::*;
    pub use super::z_index::*;

    // Component classes
    pub use super::component_classes::*;

    // CSS generation
    pub use super::simple_css_generation::*;

    // Theme system
    pub use super::theme::*;

    // Utilities
    pub use super::utilities::*;

    // Enhanced output
    pub use super::enhanced_output::*;

    // CSS snippets
    pub use super::css_snippets::*;
}