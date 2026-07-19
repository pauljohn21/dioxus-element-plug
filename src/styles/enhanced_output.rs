//! # Enhanced Output
//!
//! Enhanced CSS generation with theme support and complete Element Plus stylesheets.

/// Generate a complete Element Plus stylesheet
pub fn complete_element_plus_css() -> String {
    let mut css = String::new();
    
    // CSS Reset
    css.push_str(super::css_snippets::css_reset());
    css.push_str("\n\n");
    
    // Utility classes
    css.push_str(super::css_snippets::utility_classes());
    css.push_str("\n\n");
    
    // Grid system
    css.push_str(super::css_snippets::grid_system());
    css.push_str("\n\n");
    
    // All component styles
    css.push_str(&super::simple_css_generation::all_styles());
    
    css
}

/// Generate theme-specific CSS variables
pub fn theme_css_variables(theme: &crate::style_system::Theme) -> String {
    crate::style_system::generate_css_variables(theme)
}

/// Generate comprehensive Element Plus setup
pub fn setup_complete_element_plus() -> String {
    let theme = crate::style_system::Theme::default();
    let mut setup = String::new();
    
    setup.push_str(&theme_css_variables(&theme));
    setup.push_str("\n\n");
    setup.push_str(&complete_element_plus_css());
    
    setup
}