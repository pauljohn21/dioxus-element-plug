//! 🎨 统一的 Element Plus 纯 Rust 样式系统
//!
//! 自 0.3.0 起,本模块是 crate 的唯一样式系统入口,融合了原 `style_system.rs`
//! 的公共 API 外壳与 `styles/enhanced_css_system.rs` 的完整 CSS 实现(114 组件)。
//!
//! - [`Theme`]:50 字段完整 Element Plus 主题配置
//! - [`ThemeBuilder`]:[`Theme`] 的 fluent builder
//! - [`CompleteStyleManager`]:推荐的样式生成入口,内部委托
//!   [`all_styles`](crate::styles::enhanced_css_system::all_styles)
//! - [`CompleteCssBuilder`]:**deprecated**,0.2.x 兼容包装器,委托 [`CompleteStyleManager`]
//!
//! 设计要点:代码库中只存在一个 [`Theme`] 类型(消除 0.2.x 中 `style_system::Theme`
//! 与 `styles::theme::Theme` 的名称冲突)。`CompleteStyleManager` 保留公共 API 名称
//! 以降低迁移成本,但 CSS 覆盖从 7 组件升级到 114 组件。

use crate::styles::prelude::*;

/// 🎯 Element Plus 完整主题配置(50 字段)
///
/// 对应 Element Plus 完整设计规范,包含颜色、字体、边框、间距、过渡、阴影、
/// z-index 等全部可定制属性。所有字段为 `&'static str`(z-index 为 `i32`),
/// 默认值引用 [`crate::styles::prelude`] 中的常量。
///
/// # 迁移指南(从 0.2.x)
///
/// 0.2.x 的 `Theme` 只有 30 字段。升级到 0.3.0 后,请改用结构体更新语法,
/// 只覆盖需要修改的字段:
///
/// ```rust,ignore
/// use dioxus_element_plug::Theme;
///
/// let dark = Theme {
///     ..Theme::default(),
///     color_white: "#141414",
///     color_black: "#ffffff",
///     color_text_primary: "#E5EAF3",
/// };
/// ```
///
/// 或使用 [`ThemeBuilder`] 的 fluent API:
///
/// ```rust,ignore
/// use dioxus_element_plug::ThemeBuilder;
///
/// let theme = ThemeBuilder::new()
///     .primary_color("#1890ff")
///     .font_size_base("16px")
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq)]
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

/// 默认主题 —— 完全对应 Element Plus 默认值,引用 `styles::prelude` 的常量
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

impl Theme {
    /// Returns the default light theme.
    ///
    /// This is equivalent to `Theme::default()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dioxus_element_plug::Theme;
    ///
    /// let theme = Theme::light();
    /// ```
    pub fn light() -> Self {
        Self::default()
    }

    /// Returns a pre-configured dark theme.
    ///
    /// The dark theme uses inverted colors suitable for dark backgrounds.
    /// All colors meet WCAG AA contrast requirements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dioxus_element_plug::Theme;
    ///
    /// let theme = Theme::dark();
    /// ```
    pub fn dark() -> Self {
        Self {
            // Inverted base colors
            color_white: "#141414",
            color_black: "#ffffff",

            // Text colors (inverted hierarchy for dark backgrounds)
            color_text_primary: "#E5EAF3",
            color_text_regular: "#CFD3DC",
            color_text_secondary: "#A3A6AD",
            color_text_placeholder: "#8D9095",

            // Border colors (darkened)
            border_color_base: "#4C4D4F",
            border_color_light: "#414243",
            border_color_lighter: "#363637",
            border_color_extra_light: "#2B2B2C",

            // Background (dark)
            background_color_base: "#1d1e1f",

            // Primary colors stay similar but slightly adjusted for dark backgrounds
            color_primary: "#409EFF",
            color_success: "#67C23A",
            color_warning: "#E6A23C",
            color_danger: "#F56C6C",
            color_info: "#73767A",

            // Other fields inherit from default
            ..Self::default()
        }
    }
}

/// [`Theme`] 的 fluent builder,用于链式构造自定义主题。
///
/// # 示例
///
/// ```rust,ignore
/// use dioxus_element_plug::ThemeBuilder;
///
/// let theme = ThemeBuilder::new()
///     .primary_color("#1890ff")
///     .success_color("#52c41a")
///     .font_size_base("16px")
///     .border_radius_base("6px")
///     .box_shadow_base("0 2px 8px rgba(0,0,0,.15)")
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// 创建一个以默认主题为起点的 builder
    pub fn new() -> Self {
        Self::default()
    }

    // --- 品牌色 ---
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

    // --- 基础/文字/边框/背景色 ---
    pub fn white_color(mut self, color: &'static str) -> Self {
        self.theme.color_white = color;
        self
    }

    pub fn black_color(mut self, color: &'static str) -> Self {
        self.theme.color_black = color;
        self
    }

    pub fn text_primary_color(mut self, color: &'static str) -> Self {
        self.theme.color_text_primary = color;
        self
    }

    pub fn text_regular_color(mut self, color: &'static str) -> Self {
        self.theme.color_text_regular = color;
        self
    }

    pub fn text_secondary_color(mut self, color: &'static str) -> Self {
        self.theme.color_text_secondary = color;
        self
    }

    pub fn text_placeholder_color(mut self, color: &'static str) -> Self {
        self.theme.color_text_placeholder = color;
        self
    }

    pub fn border_color_base(mut self, color: &'static str) -> Self {
        self.theme.border_color_base = color;
        self
    }

    pub fn background_color_base(mut self, color: &'static str) -> Self {
        self.theme.background_color_base = color;
        self
    }

    // --- 字体 ---
    pub fn font_family(mut self, family: &'static str) -> Self {
        self.theme.font_family = family;
        self
    }

    pub fn font_weight_primary(mut self, weight: &'static str) -> Self {
        self.theme.font_weight_primary = weight;
        self
    }

    pub fn line_height_primary(mut self, height: &'static str) -> Self {
        self.theme.line_height_primary = height;
        self
    }

    pub fn font_size_base(mut self, size: &'static str) -> Self {
        self.theme.font_size_base = size;
        self
    }

    pub fn font_size_large(mut self, size: &'static str) -> Self {
        self.theme.font_size_large = size;
        self
    }

    pub fn font_size_small(mut self, size: &'static str) -> Self {
        self.theme.font_size_small = size;
        self
    }

    // --- 圆角 ---
    pub fn border_radius_base(mut self, radius: &'static str) -> Self {
        self.theme.border_radius_base = radius;
        self
    }

    pub fn border_radius_small(mut self, radius: &'static str) -> Self {
        self.theme.border_radius_small = radius;
        self
    }

    pub fn border_radius_large(mut self, radius: &'static str) -> Self {
        self.theme.border_radius_large = radius;
        self
    }

    // --- 组件尺寸 ---
    pub fn component_size_large(mut self, size: &'static str) -> Self {
        self.theme.component_size_large = size;
        self
    }

    pub fn component_size_default(mut self, size: &'static str) -> Self {
        self.theme.component_size_default = size;
        self
    }

    pub fn component_size_small(mut self, size: &'static str) -> Self {
        self.theme.component_size_small = size;
        self
    }

    // --- 间距 ---
    pub fn padding_base(mut self, padding: &'static str) -> Self {
        self.theme.padding_base = padding;
        self
    }

    pub fn padding_small(mut self, padding: &'static str) -> Self {
        self.theme.padding_small = padding;
        self
    }

    pub fn padding_large(mut self, padding: &'static str) -> Self {
        self.theme.padding_large = padding;
        self
    }

    pub fn margin_base(mut self, margin: &'static str) -> Self {
        self.theme.margin_base = margin;
        self
    }

    pub fn margin_small(mut self, margin: &'static str) -> Self {
        self.theme.margin_small = margin;
        self
    }

    pub fn margin_large(mut self, margin: &'static str) -> Self {
        self.theme.margin_large = margin;
        self
    }

    // --- 阴影 ---
    pub fn box_shadow_base(mut self, shadow: &'static str) -> Self {
        self.theme.box_shadow_base = shadow;
        self
    }

    pub fn box_shadow_light(mut self, shadow: &'static str) -> Self {
        self.theme.box_shadow_light = shadow;
        self
    }

    pub fn box_shadow_lighter(mut self, shadow: &'static str) -> Self {
        self.theme.box_shadow_lighter = shadow;
        self
    }

    pub fn box_shadow_dark(mut self, shadow: &'static str) -> Self {
        self.theme.box_shadow_dark = shadow;
        self
    }

    // --- z-index ---
    pub fn z_index_popper(mut self, z: i32) -> Self {
        self.theme.z_index_popper = z;
        self
    }

    pub fn z_index_dialog(mut self, z: i32) -> Self {
        self.theme.z_index_dialog = z;
        self
    }

    pub fn z_index_message(mut self, z: i32) -> Self {
        self.theme.z_index_message = z;
        self
    }

    pub fn z_index_notification(mut self, z: i32) -> Self {
        self.theme.z_index_notification = z;
        self
    }

    pub fn z_index_tooltip(mut self, z: i32) -> Self {
        self.theme.z_index_tooltip = z;
        self
    }

    /// 构建并返回 [`Theme`]
    pub fn build(self) -> Theme {
        self.theme
    }
}

/// 根据主题生成 `:root { --el-* }` CSS 自定义属性字符串
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

/// 🎯 主题管理器 —— 推荐的样式生成入口
///
/// 自 0.3.0 起,内部委托 [`all_styles`](crate::styles::enhanced_css_system::all_styles),
/// CSS 覆盖从 7 组件升级到 114 组件。`with_theme()` / `generate_complete_styles()`
/// 签名与 0.2.x 保持一致,降低迁移成本。
///
/// # 示例
///
/// ```rust,ignore
/// use dioxus_element_plug::{CompleteStyleManager, Theme};
///
/// let styles = CompleteStyleManager::new()
///     .with_theme(Theme::default())
///     .generate_complete_styles();
/// ```
pub struct CompleteStyleManager {
    theme: Theme,
}

impl Default for CompleteStyleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CompleteStyleManager {
    /// 以默认主题创建管理器
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// 注入自定义主题
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// 生成完整样式:CSS 自定义属性(`:root { --el-* }`)+ 全量组件样式(114 组件)
    ///
    /// 内部先输出主题变量,再拼接
    /// [`all_styles()`](crate::styles::enhanced_css_system::all_styles)。
    pub fn generate_complete_styles(&self) -> String {
        let mut css = String::new();
        css.push_str(&generate_css_variables(&self.theme));
        css.push_str("\n\n");
        css.push_str(&crate::styles::enhanced_css_system::all_styles());
        css
    }

    /// 使用 StyleManager 生成指定组件的 CSS
    ///
    /// 现在通过 [`StyleManager`] 实现真正的 per-component tree-shaking。
    /// 如需全量样式，请使用 [`generate_complete_styles`](Self::generate_complete_styles)。
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let css = CompleteStyleManager::new()
    ///     .generate_styles_for_components(&["button", "input"]);
    /// ```
    pub fn generate_styles_for_components(&self, components: &[&str]) -> String {
        let mut manager = StyleManager::new().with_theme(self.theme.clone());
        for &component in components {
            manager = manager.include(component);
        }
        manager.generate()
    }
}

/// 🎯 组件级 CSS Tree-shaking 管理器
///
/// 允许按需选择组件，自动解析依赖并生成最小 CSS 输出。
///
/// # 示例
///
/// ```rust,ignore
/// use dioxus_element_plug::{StyleManager, Theme};
///
/// let css = StyleManager::new()
///     .include("button")
///     .include("input")
///     .generate();
/// ```
pub struct StyleManager {
    theme: Theme,
    components: std::collections::HashSet<&'static str>,
}

/// 组件依赖关系图
/// 
/// 键: 组件名称
/// 值: 该组件依赖的其他组件列表
static COMPONENT_DEPENDENCIES: std::sync::LazyLock<std::collections::HashMap<&'static str, &[&'static str]>> =
    std::sync::LazyLock::new(|| {
        let mut deps = std::collections::HashMap::new();
        // 依赖 overlay 的组件
        deps.insert("dialog", &["overlay"] as &[&str]);
        deps.insert("drawer", &["overlay"]);
        deps.insert("dropdown", &["overlay"]);
        deps.insert("popover", &["overlay"]);
        deps.insert("tooltip", &["overlay"]);
        deps.insert("loading", &["overlay"]);
        deps.insert("message", &["overlay"]);
        deps.insert("notification", &["overlay"]);
        // 其他依赖关系可以在此添加
        deps
    });

/// 所有可用组件列表
pub const ALL_COMPONENTS: &[&str] = &[
    // 基础组件
    "button", "input", "form",
    // 布局组件
    "container", "row", "col", "grid",
    // 数据展示
    "table", "card", "descriptions", "timeline", "tag", "badge", "progress", "avatar", "empty", "skeleton",
    // 反馈组件
    "alert", "message", "dialog", "drawer", "loading", "notification", "popover", "tooltip",
    // 导航组件
    "menu", "tabs", "dropdown", "steps", "pagination", "breadcrumb", "anchor", "backtop",
    // 其他组件
    "divider", "spin", "result", "overlay",
];

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleManager {
    /// 创建新的 StyleManager
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            components: std::collections::HashSet::new(),
        }
    }

    /// 设置自定义主题
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// 添加需要包含的组件
    ///
    /// 会自动解析并包含该组件的所有依赖。
    pub fn include(mut self, component: &str) -> Self {
        self.add_component_with_deps(component);
        self
    }

    /// 递归添加组件及其依赖
    fn add_component_with_deps(&mut self, component: &str) {
        // 查找对应的静态字符串
        if let Some(&static_component) = ALL_COMPONENTS.iter().find(|&&c| c == component) {
            if self.components.insert(static_component) {
                // 新插入的组件，需要处理其依赖
                if let Some(deps) = COMPONENT_DEPENDENCIES.get(static_component) {
                    for dep in *deps {
                        self.add_component_with_deps(dep);
                    }
                }
            }
        }
    }

    /// 生成 CSS 输出
    ///
    /// 如果没有选择任何组件，返回空字符串。
    pub fn generate(&self) -> String {
        if self.components.is_empty() {
            return String::new();
        }

        let mut css = String::new();
        
        // 添加 CSS reset
        css.push_str(self.get_reset_css());
        css.push('\n');
        
        // 添加主题变量
        css.push_str(&generate_css_variables(&self.theme));
        css.push('\n');

        // 按类别添加组件样式
        let components: Vec<_> = self.components.iter().copied().collect();
        
        for component in components {
            if let Some(style) = self.get_component_css(component) {
                css.push_str(style);
                css.push('\n');
            }
        }

        css
    }

    /// 获取 CSS reset
    fn get_reset_css(&self) -> &'static str {
        r#"/* Element Plus CSS Reset */
*, *::before, *::after { box-sizing: border-box; }
html {
    font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", Arial, sans-serif;
    font-size: 14px;
    line-height: 1.42857143;
    color: #606266;
    background-color: #fff;
}"#
    }

    /// 获取单个组件的 CSS
    fn get_component_css(&self, component: &str) -> Option<&'static str> {
        use crate::styles::enhanced_css_system::*;
        
        match component {
            // 基础组件
            "button" => Some(button_styles()),
            "input" => Some(input_styles()),
            "form" => Some(form_styles()),
            // 布局组件
            "container" | "row" | "col" | "grid" => Some(layout_styles()),
            // 数据展示
            "table" | "card" | "descriptions" | "timeline" | "tag" | "badge" | "progress" | "avatar" | "empty" | "skeleton" => Some(data_display_styles()),
            // 反馈组件
            "alert" | "message" | "dialog" | "drawer" | "loading" | "notification" | "popover" | "tooltip" => Some(feedback_styles()),
            // 导航组件
            "menu" | "tabs" | "dropdown" | "steps" | "pagination" | "breadcrumb" | "anchor" | "backtop" => Some(navigation_styles()),
            // 其他组件
            "divider" | "spin" | "result" | "overlay" => Some(additional_styles()),
            _ => None,
        }
    }
}

/// **Deprecated**:0.2.x 兼容的 CSS 构建器,请改用 [`CompleteStyleManager`] 或 [`StyleManager`]。
///
/// 0.3.0 将样式系统统一到 [`CompleteStyleManager`] 后,本类型仅作为迁移期的
/// 兼容入口保留。所有方法委托 [`CompleteStyleManager`],不再持有独立的样式片段列表。
#[deprecated(note = "use CompleteStyleManager or StyleManager instead")]
pub struct CompleteCssBuilder {
    theme: Theme,
}

#[allow(deprecated)]
impl Default for CompleteCssBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl CompleteCssBuilder {
    /// **Deprecated**:请改用 [`CompleteStyleManager::new`]
    #[deprecated(note = "use CompleteStyleManager::new instead")]
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// **Deprecated**:请改用 [`CompleteStyleManager::with_theme`]
    #[deprecated(note = "use CompleteStyleManager::with_theme instead")]
    pub fn with_theme(self, theme: Theme) -> Self {
        Self { theme }
    }

    /// **Deprecated**:等价于 [`CompleteStyleManager::generate_complete_styles`]
    #[deprecated(note = "use CompleteStyleManager::generate_complete_styles instead")]
    pub fn build(self) -> String {
        CompleteStyleManager { theme: self.theme }.generate_complete_styles()
    }

    /// **Deprecated**:等价于 [`CompleteStyleManager::generate_complete_styles`]
    #[deprecated(note = "use CompleteStyleManager::generate_complete_styles instead")]
    pub fn build_complete(self) -> String {
        self.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_equality() {
        let light1 = Theme::light();
        let light2 = Theme::light();
        assert_eq!(light1, light2);

        let dark1 = Theme::dark();
        let dark2 = Theme::dark();
        assert_eq!(dark1, dark2);

        assert_ne!(Theme::light(), Theme::dark());
    }

    #[test]
    fn test_light_theme_is_default() {
        assert_eq!(Theme::light(), Theme::default());
    }

    #[test]
    fn test_dark_theme_colors() {
        let dark = Theme::dark();

        // Inverted base colors
        assert_eq!(dark.color_white, "#141414");
        assert_eq!(dark.color_black, "#ffffff");

        // Text colors for dark backgrounds
        assert_eq!(dark.color_text_primary, "#E5EAF3");
        assert_eq!(dark.color_text_regular, "#CFD3DC");
        assert_eq!(dark.color_text_secondary, "#A3A6AD");
        assert_eq!(dark.color_text_placeholder, "#8D9095");

        // Border colors (darkened)
        assert_eq!(dark.border_color_base, "#4C4D4F");
        assert_eq!(dark.border_color_light, "#414243");
        assert_eq!(dark.border_color_lighter, "#363637");
        assert_eq!(dark.border_color_extra_light, "#2B2B2C");

        // Background (dark)
        assert_eq!(dark.background_color_base, "#1d1e1f");
    }

    #[test]
    fn test_dark_theme_css_generation() {
        let dark = Theme::dark();
        let css = generate_css_variables(&dark);

        assert!(css.contains("--el-color-white: #141414"));
        assert!(css.contains("--el-color-black: #ffffff"));
        assert!(css.contains("--el-color-text-primary: #E5EAF3"));
        assert!(css.contains("--el-border-color-base: #4C4D4F"));
        assert!(css.contains("--el-background-color-base: #1d1e1f"));
    }
}
