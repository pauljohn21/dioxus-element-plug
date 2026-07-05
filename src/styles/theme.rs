//! # Theme System
//!
//! Theme definitions and builders for Element Plus style customization.

use crate::styles::prelude::*;

/// Complete Element Plus theme with all customizable properties
#[derive(Debug, Clone)]
pub struct Theme {
    // Colors
    pub color_primary: &'static str,
    pub color_success: &'static str,
    pub color_warning: &'static str,
    pub color_danger: &'static str,
    pub color_info: &'static str,
    pub color_white: &'static str,
    pub color_black: &'static str,
    pub color_text_primary: &'static str,
    pub color_text_regular: &'static str,
    pub color_text_secondary: &'static str,
    pub color_text_placeholder: &'static str,
    pub border_color_base: &'static str,
    pub border_color_light: &'static str,
    pub border_color_lighter: &'static str,
    pub border_color_extra_light: &'static str,
    pub background_color_base: &'static str,

    // Font sizes
    pub font_size_extra_large: &'static str,
    pub font_size_large: &'static str,
    pub font_size_medium: &'static str,
    pub font_size_base: &'static str,
    pub font_size_small: &'static str,
    pub font_size_extra_small: &'static str,

    // Font family
    pub font_family: &'static str,
    pub font_weight_primary: &'static str,
    pub line_height_primary: &'static str,

    // Borders
    pub border_radius_base: &'static str,
    pub border_radius_small: &'static str,
    pub border_radius_circle: &'static str,
    pub border_radius_large: &'static str,

    // Component sizes
    pub component_size_large: &'static str,
    pub component_size_default: &'static str,
    pub component_size_small: &'static str,

    // Spacing
    pub padding_base: &'static str,
    pub padding_small: &'static str,
    pub padding_large: &'static str,
    pub margin_base: &'static str,
    pub margin_small: &'static str,
    pub margin_large: &'static str,

    // Transitions
    pub transition_duration_slow: &'static str,
    pub transition_duration_base: &'static str,
    pub transition_duration_fast: &'static str,
    pub all_transition: &'static str,
    pub fade_transition: &'static str,
    pub border_transition_base: &'static str,
    pub color_transition_base: &'static str,
    pub cubic_bezier_primary: &'static str,
    pub cubic_bezier_secondary: &'static str,

    // Shadows
    pub box_shadow_base: &'static str,
    pub box_shadow_light: &'static str,
    pub box_shadow_lighter: &'static str,
    pub box_shadow_dark: &'static str,

    // Z-index
    pub z_index_base: i32,
    pub z_index_popper: i32,
    pub z_index_overlay: i32,
    pub z_index_dialog: i32,
    pub z_index_message: i32,
    pub z_index_notification: i32,
    pub z_index_tooltip: i32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Colors
            color_primary: COLOR_PRIMARY,
            color_success: COLOR_SUCCESS,
            color_warning: COLOR_WARNING,
            color_danger: COLOR_DANGER,
            color_info: COLOR_INFO,
            color_white: COLOR_WHITE,
            color_black: COLOR_BLACK,
            color_text_primary: COLOR_TEXT_PRIMARY,
            color_text_regular: COLOR_TEXT_REGULAR,
            color_text_secondary: COLOR_TEXT_SECONDARY,
            color_text_placeholder: COLOR_TEXT_PLACEHOLDER,
            border_color_base: BORDER_COLOR_BASE,
            border_color_light: BORDER_COLOR_LIGHT,
            border_color_lighter: BORDER_COLOR_LIGHTER,
            border_color_extra_light: BORDER_COLOR_EXTRA_LIGHT,
            background_color_base: BACKGROUND_COLOR_BASE,

            // Font sizes
            font_size_extra_large: FONT_SIZE_EXTRA_LARGE,
            font_size_large: FONT_SIZE_LARGE,
            font_size_medium: FONT_SIZE_MEDIUM,
            font_size_base: FONT_SIZE_BASE,
            font_size_small: FONT_SIZE_SMALL,
            font_size_extra_small: FONT_SIZE_EXTRA_SMALL,

            // Font family
            font_family: FONT_FAMILY,
            font_weight_primary: FONT_WEIGHT_PRIMARY,
            line_height_primary: FONT_LINE_HEIGHT_PRIMARY,

            // Borders
            border_radius_base: BORDER_RADIUS_BASE,
            border_radius_small: BORDER_RADIUS_SMALL,
            border_radius_circle: BORDER_RADIUS_CIRCLE,
            border_radius_large: BORDER_RADIUS_LARGE,

            // Component sizes
            component_size_large: COMPONENT_SIZE_LARGE,
            component_size_default: COMPONENT_SIZE_DEFAULT,
            component_size_small: COMPONENT_SIZE_SMALL,

            // Spacing
            padding_base: SPACING_MD,
            padding_small: SPACING_SM,
            padding_large: SPACING_LG,
            margin_base: SPACING_MD,
            margin_small: SPACING_SM,
            margin_large: SPACING_LG,

            // Transitions
            transition_duration_slow: TRANSITION_DURATION_SLOW,
            transition_duration_base: TRANSITION_DURATION_BASE,
            transition_duration_fast: TRANSITION_DURATION_FAST,
            all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
            fade_transition: "opacity .3s cubic-bezier(.55,0,.1,1)",
            border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
            color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
            cubic_bezier_primary: "cubic-bezier(.645,.045,.355,1)",
            cubic_bezier_secondary: "cubic-bezier(.23,1,.32,1)",

            // Shadows
            box_shadow_base: BOX_SHADOW_BASE,
            box_shadow_light: BOX_SHADOW_LIGHT,
            box_shadow_lighter: BOX_SHADOW_LIGHTER,
            box_shadow_dark: BOX_SHADOW_DARK,

            // Z-index
            z_index_base: 100,
            z_index_popper: 2000,
            z_index_overlay: 2000,
            z_index_dialog: 2001,
            z_index_message: 2002,
            z_index_notification: 2003,
            z_index_tooltip: 2004,
        }
    }
}

/// Theme builder for custom theme creation
#[derive(Debug, Default)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn primary_color(mut self, color: &'static str) -> Self {
        self.theme.color_primary = color;
        self
    }

    pub fn success_color(mut self, color: &'static str) -> Self {
        self.theme.color_success = color;
        self
    }

    pub fn warning_color(mut self, color: &'static str) -> Self {
        self.theme.color_warning = color;
        self
    }

    pub fn danger_color(mut self, color: &'static str) -> Self {
        self.theme.color_danger = color;
        self
    }

    pub fn info_color(mut self, color: &'static str) -> Self {
        self.theme.color_info = color;
        self
    }

    pub fn font_size_base(mut self, size: &'static str) -> Self {
        self.theme.font_size_base = size;
        self
    }

    pub fn border_radius_base(mut self, radius: &'static str) -> Self {
        self.theme.border_radius_base = radius;
        self
    }

    pub fn build(self) -> Theme {
        self.theme
    }
}

/// Generate CSS variables string for theme
pub fn generate_css_variables(theme: &Theme) -> String {
    format!(
        ":root {{\n  
  /* Colors */\n  
  --el-color-primary: {};\n  
  --el-color-success: {};\n  
  --el-color-warning: {};\n  
  --el-color-danger: {};\n  
  --el-color-info: {};\n  
  --el-color-white: {};\n  
  --el-color-black: {};\n
  /* Text colors */\n  
  --el-color-text-primary: {};\n  
  --el-color-text-regular: {};\n  
  --el-color-text-secondary: {};\n  
  --el-color-text-placeholder: {};\n
  /* Border colors */\n  
  --el-border-color-base: {};\n  
  --el-border-color-light: {};\n  
  --el-border-color-lighter: {};\n  
  --el-border-color-extra-light: {};\n
  /* Background */\n  
  --el-background-color-base: {};\n
  /* Font sizes */\n  
  --el-font-size-extra-large: {};\n  
  --el-font-size-large: {};\n  
  --el-font-size-medium: {};\n  
  --el-font-size-base: {};\n  
  --el-font-size-small: {};\n  
  --el-font-size-extra-small: {};\n
  /* Font */\n  
  --el-font-family: {};\n  
  --el-font-weight-primary: {};\n  
  --el-line-height-primary: {};\n
  /* Border radius */\n  
  --el-border-radius-base: {};\n  
  --el-border-radius-small: {};\n  
  --el-border-radius-circle: {};\n  
  --el-border-radius-large: {};\n
  /* Box shadows */\n  
  --el-box-shadow-base: {};\n  
  --el-box-shadow-light: {};\n  
  --el-box-shadow-lighter: {};\n  
  --el-box-shadow-dark: {};\n
  /* Transition */\n  
  --el-transition-duration-slow: {};\n  
  --el-transition-duration-base: {};\n  
  --el-transition-duration-fast: {};\n\n}}",
        theme.color_primary,
        theme.color_success,
        theme.color_warning,
        theme.color_danger,
        theme.color_info,
        theme.color_white,
        theme.color_black,
        theme.color_text_primary,
        theme.color_text_regular,
        theme.color_text_secondary,
        theme.color_text_placeholder,
        theme.border_color_base,
        theme.border_color_light,
        theme.border_color_lighter,
        theme.border_color_extra_light,
        theme.background_color_base,
        theme.font_size_extra_large,
        theme.font_size_large,
        theme.font_size_medium,
        theme.font_size_base,
        theme.font_size_small,
        theme.font_size_extra_small,
        theme.font_family,
        theme.font_weight_primary,
        theme.line_height_primary,
        theme.border_radius_base,
        theme.border_radius_small,
        theme.border_radius_circle,
        theme.border_radius_large,
        theme.box_shadow_base,
        theme.box_shadow_light,
        theme.box_shadow_lighter,
        theme.box_shadow_dark,
        theme.transition_duration_slow,
        theme.transition_duration_base,
        theme.transition_duration_fast
    )
}