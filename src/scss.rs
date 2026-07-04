//! # SCSS Asset Utilities
//!
//! This module provides utilities for working with SCSS assets in Dioxus applications
//! using the built-in manganis support in Dioxus 0.7+.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dioxus_element_plug::scss::asset;
//! use dioxus_element_plug::scss::theme_variables;
//!
//! // Reference a SCSS file for automatic compilation
//! static STYLES: Asset = asset!("/assets/theme.scss");
//!
//! // Access theme variables in Rust
//! let primary_color = theme_variables::PRIMARY;
//! ```

/// Re-export the manganis asset macro for convenient access
pub use manganis::{self, asset};

/// Theme color constants that match the SCSS variables
/// These can be used in both SCSS and Rust code for consistency
pub mod theme_variables {
    /// Primary brand color
    pub const PRIMARY: &str = "#409EFF";
    /// Success state color
    pub const SUCCESS: &str = "#67C23A";
    /// Warning state color
    pub const WARNING: &str = "#E6A23C";
    /// Danger/error state color
    pub const DANGER: &str = "#F56C6C";
    /// Information state color
    pub const INFO: &str = "#909399";
    
    /// Base text color
    pub const TEXT_PRIMARY: &str = "#303133";
    /// Secondary text color
    pub const TEXT_SECONDARY: &str = "#606266";
    /// Placeholder text color
    pub const TEXT_PLACEHOLDER: &str = "#C0C4CC";
    
    /// Base border color
    pub const BORDER_BASE: &str = "#DCDFE6";
    /// Light border color
    pub const BORDER_LIGHT: &str = "#E4E7ED";
    /// Lighter border color
    pub const BORDER_LIGHTER: &str = "#EBEEF5";
    
    /// Background color base
    pub const BACKGROUND_BASE: &str = "#F5F7FA";
    /// White background
    pub const BACKGROUND_WHITE: &str = "#FFFFFF";
    
    /// Base border radius
    pub const BORDER_RADIUS_BASE: &str = "4px";
    /// Small border radius
    pub const BORDER_RADIUS_SMALL: &str = "2px";
    /// Round border radius
    pub const BORDER_RADIUS_ROUND: &str = "20px";
    
    /// Base font size
    pub const FONT_SIZE_BASE: &str = "14px";
    /// Large font size
    pub const FONT_SIZE_LARGE: &str = "18px";
    /// Small font size
    pub const FONT_SIZE_SMALL: &str = "12px";
}

/// Typography constants
pub mod typography {
    /// Base font family (consistent with Element UI)
    pub const FONT_FAMILY_BASE: &str = "'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', 'Arial', sans-serif";
    
    /// Code font family
    pub const FONT_FAMILY_MONO: &str = "'Courier New', Courier, monospace";
    
    /// Base line height
    pub const LINE_HEIGHT_BASE: f32 = 1.5;
    
    /// Heading line height
    pub const LINE_HEIGHT_HEADING: f32 = 1.2;
}

/// Spacing utilities
pub mod spacing {
    /// Base spacing unit (4px)
    pub const UNIT: u8 = 4;
    
    /// Extra small spacing (4px)
    pub const XS: u8 = UNIT;
    /// Small spacing (8px)
    pub const SM: u8 = UNIT * 2;
    /// Medium spacing (16px)
    pub const MD: u8 = UNIT * 4;
    /// Large spacing (24px)
    pub const LG: u8 = UNIT * 6;
    /// Extra large spacing (32px)
    pub const XL: u8 = UNIT * 8;
}

/// Pre-defined SCSS assets for common use cases
/// These can be used directly in your Dioxus applications
pub mod prebuilt {
    /// Complete theme-chalk SCSS (if the file exists at this path)
    /// Usage: `static THEME: Asset = theme_chalk_scss!();`
    #[macro_export]
    macro_rules! theme_chalk_scss {
        () => {
            manganis::asset!("/scss/index.scss")
        };
    }
    
    /// Button component SCSS
    #[macro_export]
    macro_rules! button_scss {
        () => {
            manganis::asset!("/scss/components/button.scss")
        };
    }
    
    /// Input component SCSS
    #[macro_export]
    macro_rules! input_scss {
        () => {
            manganis::asset!("/scss/components/input.scss")
        };
    }
    
    /// Layout component SCSS
    #[macro_export]
    macro_rules! layout_scss {
        () => {
            manganis::asset!("/scss/layout/*.scss")
        };
    }
}

/// CSS class name utilities
/// These help ensure consistency between Rust and CSS
pub mod class_names {
    /// Button class names
    pub mod button {
        /// Base button class
        pub const BASE: &str = "el-button";
        /// Primary button variant
        pub const PRIMARY: &str = "el-button--primary";
        /// Success button variant
        pub const SUCCESS: &str = "el-button--success";
        /// Warning button variant
        pub const WARNING: &str = "el-button--warning";
        /// Danger button variant
        pub const DANGER: &str = "el-button--danger";
        /// Info button variant
        pub const INFO: &str = "el-button--info";
        /// Large button size
        pub const LARGE: &str = "el-button--large";
        /// Medium button size
        pub const MEDIUM: &str = "el-button--medium";
        /// Small button size
        pub const SMALL: &str = "el-button--small";
        /// Mini button size
        pub const MINI: &str = "el-button--mini";
    }
    
    /// Input class names
    pub mod input {
        /// Base input class
        pub const BASE: &str = "el-input";
        /// Input inner element
        pub const INNER: &str = "el-input__inner";
        /// Large input size
        pub const LARGE: &str = "el-input--large";
        /// Medium input size
        pub const MEDIUM: &str = "el-input--medium";
        /// Small input size
        pub const SMALL: &str = "el-input--small";
        /// Mini input size
        pub const MINI: &str = "el-input--mini";
    }
    
    /// Layout class names
    pub mod layout {
        /// Container base
        pub const CONTAINER: &str = "el-container";
        /// Header element
        pub const HEADER: &str = "el-header";
        /// Main content element
        pub const MAIN: &str = "el-main";
        /// Footer element
        pub const FOOTER: &str = "el-footer";
        /// Aside element
        pub const ASIDE: &str = "el-aside";
        /// Row element
        pub const ROW: &str = "el-row";
        /// Column element
        pub const COL: &str = "el-col";
    }
}

/// Helper functions for dynamic styling
pub mod helpers {
    
    /// Creates a CSS custom property string from a key-value pair
    /// 
    /// ```rust
    /// let style = css_var("--el-color-primary", "#409EFF");
    /// // Returns: "--el-color-primary: #409EFF"
    /// ```
    pub fn css_var(key: &str, value: &str) -> String {
        format!("{}: {}", key, value)
    }
    
    /// Creates a complete CSS rule from selector and properties
    /// 
    /// ```rust
    /// let rule = css_rule(".my-button", &["color: red", "background: blue"]);
    /// // Returns: ".my-button { color: red; background: blue; }"
    /// ```
    pub fn css_rule(selector: &str, properties: &[&str]) -> String {
        let props = properties.join("; ");
        format!("{} {{ {}; }}", selector, props)
    }
    
    /// Creates a responsive CSS media query
    /// 
    /// ```rust
    /// let media = css_media("(max-width: 768px)", ".mobile-hide { display: none; }");
    /// // Returns: "@media (max-width: 768px) { .mobile-hide { display: none; } }"
    /// ```
    pub fn css_media(query: &str, rules: &str) -> String {
        format!("@media {} {{ {} }}", query, rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_constants() {
        assert_eq!(theme_variables::PRIMARY, "#409EFF");
        assert_eq!(theme_variables::SUCCESS, "#67C23A");
        assert_eq!(theme_variables::WARNING, "#E6A23C");
        assert_eq!(theme_variables::DANGER, "#F56C6C");
        assert_eq!(theme_variables::INFO, "#909399");
    }
    
    #[test]
    fn test_css_helpers() {
        let var = helpers::css_var("--color", "red");
        assert_eq!(var, "--color: red");
        
        let rule = helpers::css_rule(".test", &["color: blue", "margin: 0"]);
        assert_eq!(rule, ".test { color: blue; margin: 0; }");
        
        let media = helpers::css_media("(max-width: 768px)", ".hide { display: none; }");
        assert_eq!(media, "@media (max-width: 768px) { .hide { display: none; } }");
    }
}
