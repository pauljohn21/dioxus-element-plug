//! Common utilities for component reuse
//!
//! Provides shared traits and builders to reduce boilerplate across components.

use dioxus::prelude::*;

/// Builder for constructing CSS class strings with conditional classes.
///
/// Eliminates the repetitive `vec![]` → `push` → `join(" ")` pattern.
#[derive(Debug, Clone)]
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    /// Create a new builder with a base class.
    pub fn new(base: &str) -> Self {
        Self {
            classes: vec![base.to_string()],
        }
    }

    /// Add a class unconditionally.
    pub fn add_class(mut self, class: &str) -> Self {
        if !class.is_empty() {
            self.classes.push(class.to_string());
        }
        self
    }

    /// Add a class only if `condition` is true.
    pub fn add_if(mut self, class: &str, condition: bool) -> Self {
        if condition {
            self.classes.push(class.to_string());
        }
        self
    }

    /// Add a class from an `Option<String>`.
    pub fn add_opt(mut self, class: Option<&String>) -> Self {
        if let Some(c) = class {
            self.classes.push(c.clone());
        }
        self
    }

    /// Add a class from an `Option<&str>`.
    pub fn add_opt_str(mut self, class: Option<&str>) -> Self {
        if let Some(c) = class {
            self.classes.push(c.to_string());
        }
        self
    }

    /// Build the final class string.
    pub fn build(self) -> String {
        self.classes.join(" ")
    }
}

/// Extract the style string from an `Option<String>`.
///
/// Returns empty string if `None`.
#[inline]
pub fn style_str(style: &Option<String>) -> String {
    style.clone().unwrap_or_default()
}

/// Call an optional event handler with the given event.
#[inline]
pub fn fire_event<E: 'static>(handler: &Option<EventHandler<E>>, event: E) {
    if let Some(h) = handler {
        h.call(event);
    }
}
