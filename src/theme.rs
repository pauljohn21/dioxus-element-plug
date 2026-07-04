/// Theme constants and CSS utilities for theme-chalk
///
/// This module provides constants and utilities related to the theme-chalk CSS framework.

/// CSS classes and constants from theme-chalk
pub mod classes {
    // Button variants
    pub const BUTTON_PRIMARY: &str = "el-button--primary";
    pub const BUTTON_SUCCESS: &str = "el-button--success";
    pub const BUTTON_WARNING: &str = "el-button--warning";
    pub const BUTTON_DANGER: &str = "el-button--danger";
    pub const BUTTON_INFO: &str = "el-button--info";

    // Button sizes
    pub const BUTTON_LARGE: &str = "el-button--large";
    pub const BUTTON_MEDIUM: &str = "el-button--medium";
    pub const BUTTON_SMALL: &str = "el-button--small";
    pub const BUTTON_MINI: &str = "el-button--mini";

    // Input sizes
    pub const INPUT_LARGE: &str = "el-input--large";
    pub const INPUT_MEDIUM: &str = "el-input--medium";
    pub const INPUT_SMALL: &str = "el-input--small";
    pub const INPUT_MINI: &str = "el-input--mini";

    // Common states
    pub const IS_DISABLED: &str = "is-disabled";
    pub const IS_ACTIVE: &str = "is-active";
    pub const IS_FOCUS: &str = "is-focus";
    pub const IS_HOVER: &str = "is-hover";

    // Layout
    pub const CONTAINER: &str = "el-container";
    pub const HEADER: &str = "el-header";
    pub const ASIDE: &str = "el-aside";
    pub const MAIN: &str = "el-main";
    pub const FOOTER: &str = "el-footer";

    // Grid system
    pub const ROW: &str = "el-row";
    pub const COL: &str = "el-col";
}

/// Color palette from theme-chalk
pub mod colors {
    pub const PRIMARY: &str = "#409EFF";
    pub const SUCCESS: &str = "#67C23A";
    pub const WARNING: &str = "#E6A23C";
    pub const DANGER: &str = "#F56C6C";
    pub const INFO: &str = "#909399";
    pub const WHITE: &str = "#FFFFFF";
    pub const BLACK: &str = "#000000";
    pub const TEXT_PRIMARY: &str = "#303133";
    pub const TEXT_REGULAR: &str = "#606266";
    pub const TEXT_SECONDARY: &str = "#909399";
    pub const TEXT_PLACEHOLDER: &str = "#C0C4CC";
}

/// Typography constants
pub mod typography {
    pub const FONT_SIZE_BASE: &str = "14px";
    pub const FONT_SIZE_LARGE: &str = "18px";
    pub const FONT_SIZE_SMALL: &str = "13px";
    pub const FONT_WEIGHT_PRIMARY: &str = "500";
    pub const LINE_HEIGHT_PRIMARY: &str = "24px";
}

/// Spacing constants
pub mod spacing {
    pub const PADDING_SMALL: &str = "8px";
    pub const PADDING_MEDIUM: &str = "16px";
    pub const PADDING_LARGE: &str = "24px";
    pub const MARGIN_SMALL: &str = "8px";
    pub const MARGIN_MEDIUM: &str = "16px";
    pub const MARGIN_LARGE: &str = "24px";
}

/// Border constants
pub mod borders {
    pub const RADIUS_BASE: &str = "4px";
    pub const RADIUS_SMALL: &str = "2px";
    pub const WIDTH_BASE: &str = "1px";
    pub const COLOR_BASE: &str = "#DCDFE6";
    pub const COLOR_LIGHT: &str = "#E4E7ED";
    pub const COLOR_LIGHTER: &str = "#EBEEF5";
}

/// Generates the CSS file path for the compiled theme-chalk
pub fn get_theme_css_path() -> &'static str {
    "/dist/theme-chalk.css"
}

/// Theme configuration for the component library
#[derive(Clone, Debug)]
pub struct Theme {
    /// Whether to include the base CSS reset
    pub include_reset: bool,
    /// Custom CSS to inject
    pub custom_css: Option<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            include_reset: true,
            custom_css: None,
        }
    }
}
