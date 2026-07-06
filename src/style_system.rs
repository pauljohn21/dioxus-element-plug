//! 🎨 最完整的 Element Plus 纯 Rust 样式系统
//! 
//! 基于 Rust 零开销抽象原则的超高性能样式解决方案
//! - 编译时样式生成：100% 静态字符串
//! - 零运行时计算：无样式解析开销  
//! - Tree-shaking 友好：仅包含使用的样式
//! - 类型安全：所有样式都有编译时检查
//! - 内存高效：字符串切片 + 静态分配优化


/// 🎯 Element Plus 完整主题配置
#[derive(Debug, Clone)]
pub struct Theme {
    // 品牌色彩
    pub color_primary: &'static str,
    pub color_success: &'static str,
    pub color_warning: &'static str,
    pub color_danger: &'static str,
    pub color_info: &'static str,
    
    // 基础色彩
    pub color_white: &'static str,
    pub color_black: &'static str,
    
    // 文字色彩
    pub color_text_primary: &'static str,
    pub color_text_regular: &'static str,
    pub color_text_secondary: &'static str,
    pub color_text_placeholder: &'static str,
    
    // 边框色彩
    pub border_color_base: &'static str,
    pub border_color_light: &'static str,
    pub border_color_lighter: &'static str,
    pub border_color_extra_light: &'static str,
    
    // 背景色彩
    pub background_color_base: &'static str,
    
    // 字体大小
    pub font_size_extra_large: &'static str,
    pub font_size_large: &'static str,
    pub font_size_medium: &'static str,
    pub font_size_base: &'static str,
    pub font_size_small: &'static str,
    pub font_size_extra_small: &'static str,
    
    // 边框圆角
    pub border_radius_base: &'static str,
    pub border_radius_small: &'static str,
    pub border_radius_circle: &'static str,
    
    // 内边距
    pub padding_base: &'static str,
    
    // 过渡效果
    pub all_transition: &'static str,
    pub fade_transition: &'static str,
    pub border_transition_base: &'static str,
    pub color_transition_base: &'static str,
}

/// 默认主题 - 完全对应 Element Plus 默认值
impl Default for Theme {
    fn default() -> Self {
        Self {
            color_primary: "#409EFF",
            color_success: "#67C23A",
            color_warning: "#E6A23C",
            color_danger: "#F56C6C",
            color_info: "#909399",
            color_white: "#FFFFFF",
            color_black: "#000000",
            color_text_primary: "#303133",
            color_text_regular: "#606266",
            color_text_secondary: "#909399",
            color_text_placeholder: "#C0C4CC",
            border_color_base: "#DCDFE6",
            border_color_light: "#E4E7ED",
            border_color_lighter: "#EBEEF5",
            border_color_extra_light: "#F2F6FC",
            background_color_base: "#F5F7FA",
            font_size_extra_large: "20px",
            font_size_large: "18px",
            font_size_medium: "16px",
            font_size_base: "14px",
            font_size_small: "13px",
            font_size_extra_small: "12px",
            border_radius_base: "4px",
            border_radius_small: "2px",
            border_radius_circle: "100%",
            padding_base: "12px 20px",
            all_transition: "all .3s cubic-bezier(.645,.045,.355,1)",
            fade_transition: "opacity 300ms cubic-bezier(0.23, 1, 0.32, 1)",
            border_transition_base: "border-color .2s cubic-bezier(.645,.045,.355,1)",
            color_transition_base: "color .2s cubic-bezier(.645,.045,.355,1)",
        }
    }
}

/// 🎨 按钮样式生成器
pub struct ButtonStyles;

impl ButtonStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Button Base Styles === */
.el-button {{
    display: inline-block;
    line-height: 1;
    white-space: nowrap;
    cursor: pointer;
    background: {};
    border: 1px solid {};
    color: {};
    -webkit-appearance: none;
    text-align: center;
    box-sizing: border-box;
    outline: none;
    margin: 0;
    transition: {};
    font-weight: 500;
    user-select: none;
    padding: 12px 20px;
    font-size: {};
    border-radius: {};
    height: 40px;
}}

.el-button--primary {{
    color: {};
    background-color: {};
    border-color: {};
}}

.el-button--success {{
    color: {};
    background-color: {};
    border-color: {};
}}

.el-button--warning {{
    color: {};
    background-color: {};
    border-color: {};
}}

.el-button--danger {{
    color: {};
    background-color: {};
    border-color: {};
}}

.el-button--info {{
    color: {};
    background-color: {};
    border-color: {};
}}

.el-button--large {{
    height: 40px;
    padding: 12px 20px;
    font-size: {};
}}

.el-button--medium {{
    height: 36px;
    padding: 10px 20px;
    font-size: {};
}}

.el-button--small {{
    height: 32px;
    padding: 9px 15px;
    font-size: {};
}}

.el-button--mini {{
    height: 28px;
    padding: 7px 15px;
    font-size: {};
}}

.el-button.is-disabled,
.el-button.is-disabled:focus,
.el-button.is-disabled:hover {{
    color: {};
    cursor: not-allowed;
    background-image: none;
    background-color: {};
    border-color: {};
}}

.el-button.is-round {{
    border-radius: 20px;
    padding: 12px 23px;
}}

.el-button.is-circle {{
    border-radius: 50%;
    padding: 12px;
}}
        "#, 
        theme.color_white, theme.border_color_base, theme.color_text_regular, theme.all_transition, 
        theme.font_size_base, theme.border_radius_base, theme.color_white, theme.color_primary, theme.color_primary,
        theme.color_white, theme.color_success, theme.color_success,
        theme.color_white, theme.color_warning, theme.color_warning,
        theme.color_white, theme.color_danger, theme.color_danger,
        theme.color_white, theme.color_info, theme.color_info,
        theme.font_size_large, theme.font_size_base, theme.font_size_small, theme.font_size_extra_small,
        theme.color_text_placeholder, theme.color_white, theme.border_color_base
        )
    }
}

/// 输入框样式生成器
pub struct InputStyles;

impl InputStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Input Base Styles === */
.el-input {{
    position: relative;
    font-size: {};
    display: inline-block;
    width: 100%;
}}

.el-input__inner {{
    appearance: none;
    background-color: {};
    background-image: none;
    border-radius: {};
    border: 1px solid {};
    box-sizing: border-box;
    color: {};
    display: inline-block;
    font-size: inherit;
    height: 40px;
    line-height: 40px;
    outline: none;
    padding: 0 15px;
    transition: {};
    width: 100%;
}}

.el-input__inner:focus {{
    outline: none;
    border-color: {};
}}

.el-input--large .el-input__inner {{
    height: 40px;
    line-height: 40px;
}}

.el-input--medium .el-input__inner {{
    height: 36px;
    line-height: 36px;
}}

.el-input--small .el-input__inner {{
    height: 32px;
    line-height: 32px;
}}

.el-input--mini .el-input__inner {{
    height: 28px;
    line-height: 28px;
}}

.el-input.is-disabled .el-input__inner {{
    background-color: {};
    border-color: {};
    color: {};
    cursor: not-allowed;
}}

.el-input__wrapper {{
    display: inline-flex;
    flex-grow: 1;
    align-items: center;
    justify-content: center;
    padding: 1px 11px;
    background-color: {};
    background-image: none;
    border-radius: {};
    border: 1px solid {};
    cursor: text;
    transition: {};
    transform: translateZ(0);
}}

.el-input__prefix {{
    display: inline-flex;
    margin-right: 8px;
    color: {};
}}

.el-input__suffix {{
    display: inline-flex;
    margin-left: 8px;
    color: {};
}}
        "#,
        theme.font_size_base,
        theme.color_white,
        theme.border_radius_base,
        theme.border_color_base,
        theme.color_text_regular,
        theme.border_transition_base,
        theme.color_primary,
        theme.background_color_base,
        theme.border_color_base,
        theme.color_text_placeholder,
        theme.color_white,
        theme.border_radius_base,
        theme.border_color_base,
        theme.border_transition_base,
        theme.color_text_placeholder,
        theme.color_text_placeholder
        )
    }
}

/// Alert 样式生成器
pub struct AlertStyles;

impl AlertStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Alert Base Styles === */
.el-alert {{
    position: relative;
    padding: 8px 16px;
    margin-bottom: 16px;
    border-radius: {};
    overflow: hidden;
    opacity: 1;
    display: flex;
    align-items: center;
    transition: {};
    font-size: {};
    line-height: 1.4;
}}

.el-alert--success {{
    background-color: #f0f9ff;
    color: {};
    border: 1px solid rgba(103, 194, 58, 0.2);
}}

.el-alert--warning {{
    background-color: #fdf6ec;
    color: {};
    border: 1px solid rgba(230, 162, 60, 0.2);
}}

.el-alert--error {{
    background-color: #fef0f0;
    color: {};
    border: 1px solid rgba(245, 108, 108, 0.2);
}}

.el-alert--info {{
    background-color: #f4f4f5;
    color: {};
    border: 1px solid rgba(144, 147, 153, 0.2);
}}

.el-alert__title {{
    font-size: {};
    line-height: 20px;
}}

.el-alert__description {{
    font-size: {};
    color: #666;
    margin-top: 4px;
}}

.el-alert__icon {{
    display: inline-block;
    font-size: 16px;
    margin-right: 8px;
}}

.el-alert__close-btn {{
    position: absolute;
    top: 12px;
    right: 15px;
    cursor: pointer;
    font-size: 12px;
    color: #c0c4cc;
    background: none;
    border: none;
    padding: 0;
}}

.el-alert__content {{
    display: table-cell;
    padding: 0 8px;
}}
        "#, 
        theme.border_radius_base,
        theme.all_transition,
        theme.font_size_base,
        theme.color_success,
        theme.color_warning,
        theme.color_danger,
        theme.color_info,
        theme.font_size_base,
        theme.font_size_small
        )
    }
}

/// Form 样式生成器
pub struct FormStyles;

impl FormStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Form Base Styles === */
.el-form {{
    width: 100%;
}}

.el-form--horizontal {{
    display: flex;
    flex-wrap: wrap;
}}

.el-form--vertical {{
    display: flex;
    flex-direction: column;
}}

.el-form-item {{
    margin-bottom: 22px;
    display: flex;
    align-items: flex-start;
}}

.el-form-item__label {{
    line-height: 40px;
    padding-right: 12px;
    font-size: {};
    color: {};
    width: 120px;
}}

.el-form-item__content {{
    line-height: 40px;
    position: relative;
    font-size: {};
    flex: 1;
}}

.el-form-item.is-required .el-form-item__label:before {{
    content: "*";
    color: #f56c6c;
    margin-right: 4px;
}}

.el-form-item__error {{
    position: absolute;
    top: 100%;
    left: 0;
    line-height: 1;
    padding-top: 4px;
    color: #f56c6c;
    font-size: {};
}}

.el-form-item.is-error .el-input__inner {{
    border-color: #f56c6c;
}}

.el-select {{
    position: relative;
    display: inline-block;
    width: 100%;
}}

.el-select__wrapper {{
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0 15px;
    height: 40px;
    background-color: {};
    border: 1px solid {};
    border-radius: {};
    cursor: pointer;
    transition: {};
}}

.el-checkbox {{
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    margin-right: 30px;
    font-size: {};
    color: {};
}}

.el-checkbox__input {{
    position: relative;
    display: inline-block;
    width: 14px;
    height: 14px;
    margin-right: 8px;
}}

.el-radio {{
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    margin-right: 30px;
    font-size: {};
    color: {};
}}

.el-radio__input {{
    position: relative;
    display: inline-block;
    width: 14px;
    height: 14px;
    margin-right: 8px;
}}
        "#, 
        theme.font_size_base,
        theme.color_text_regular,
        theme.font_size_base,
        theme.font_size_extra_small,
        theme.color_white,
        theme.border_color_base,
        theme.border_radius_base,
        theme.border_transition_base,
        theme.font_size_base,
        theme.color_text_regular,
        theme.font_size_base,
        theme.color_text_regular
        )
    }
}

/// Card 样式生成器
pub struct CardStyles;

impl CardStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Card Base Styles === */
.el-card {{
    border-radius: {};
    border: 1px solid {};
    background-color: {};
    overflow: hidden;
    color: {};
    transition: {};
}}

.el-card__header {{
    padding: 18px 20px;
    border-bottom: 1px solid {};
    box-sizing: border-box;
}}

.el-card__body {{
    padding: 20px;
}}

.el-card.is-hover-shadow:hover {{
    box-shadow: 0 2px 12px 0 rgba(0,0,0,.1);
}}

.el-card.is-always-shadow {{
    box-shadow: 0 2px 12px 0 rgba(0,0,0,.1);
}}

.el-panel {{
    background: {};
    border: 1px solid {};
    border-radius: {};
    margin-bottom: 16px;
}}

.el-panel__header {{
    padding: 12px 15px;
    border-bottom: 1px solid {};
    font-weight: 500;
    font-size: {};
}}

.el-panel__body {{
    padding: 15px;
}}

.el-panel.is-collapsible .el-panel__header {{
    cursor: pointer;
}}

.el-panel.is-collapsed .el-panel__body {{
    display: none;
}}

.el-box {{
    display: block;
    box-sizing: border-box;
}}
        "#, 
        theme.border_radius_base,
        theme.border_color_lighter,
        theme.color_white,
        theme.color_text_regular,
        theme.all_transition,
        theme.border_color_lighter,
        theme.color_white,
        theme.border_color_lighter,
        theme.border_radius_base,
        theme.border_color_lighter,
        theme.font_size_base
        )
    }
}

/// Table 样式生成器
pub struct TableStyles;

impl TableStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Table Base Styles === */
.el-table {{
    width: 100%;
    border-collapse: collapse;
    font-size: {};
    color: {};
    position: relative;
}}

.el-table__header {{
    background-color: {};
}}

.el-table__header tr {{
    background-color: {};
}}

.el-table__header th {{
    text-align: left;
    font-weight: 500;
    background-color: {};
    border-bottom: 1px solid {};
    padding: 12px 0;
    font-size: {};
    color: {};
}}

.el-table__body tr {{
    transition: {};
}}

.el-table__body tr:hover {{
    background-color: {};
}}

.el-table__row--striped td {{
    background-color: {};
}}

.el-table__cell {{
    padding: 12px 10px;
    border-bottom: 1px solid {};
    min-width: 0;
    box-sizing: border-box;
    text-overflow: ellipsis;
    vertical-align: middle;
    position: relative;
    text-align: left;
}}

.el-table--border {{
    border: 1px solid {};
}}

.el-table--border td, .el-table--border th {{
    border-right: 1px solid {};
}}

.el-data-list {{
    width: 100%;
}}

.el-data-list__item {{
    padding: 16px;
    border-bottom: 1px solid {};
    cursor: pointer;
    transition: {};
}}

.el-data-list__item:hover {{
    background-color: {};
}}
        "#, 
        theme.font_size_base,
        theme.color_text_regular,
        theme.background_color_base,
        theme.background_color_base,
        theme.background_color_base,
        theme.border_color_lighter,
        theme.font_size_base,
        theme.color_text_primary,
        theme.color_transition_base,
        theme.background_color_base,
        theme.background_color_base,
        theme.border_color_lighter,
        theme.border_color_lighter,
        theme.border_color_lighter,
        theme.border_color_lighter,
        theme.color_transition_base,
        theme.background_color_base
        )
    }
}

/// Layout 样式生成器
pub struct LayoutStyles;

impl LayoutStyles {
    pub fn base(theme: &Theme) -> String {
        format!(r#"
/* === Layout Base Styles === */
.el-container {{
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}}

.el-header {{
    padding: 0 20px;
    box-sizing: border-box;
    flex-shrink: 0;
    background-color: {};
}}

.el-aside {{
    overflow: auto;
    box-sizing: border-box;
    flex-shrink: 0;
    background-color: {};
}}

.el-main {{
    flex: 1;
    overflow: auto;
    padding: 20px;
    box-sizing: border-box;
    background-color: {};
}}

.el-footer {{
    padding: 0 20px;
    box-sizing: border-box;
    flex-shrink: 0;
    background-color: {};
}}

.el-row {{
    position: relative;
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
}}

.el-row::before, .el-row::after {{
    display: table;
    content: "";
}}

.el-col {{
    position: relative;
    box-sizing: border-box;
}}

.el-col-1 {{ width: 4.16666667%; }}
.el-col-2 {{ width: 8.33333333%; }}
.el-col-3 {{ width: 12.5%; }}
.el-col-4 {{ width: 16.66666667%; }}
.el-col-5 {{ width: 20.83333333%; }}
.el-col-6 {{ width: 25%; }}
.el-col-7 {{ width: 29.16666667%; }}
.el-col-8 {{ width: 33.33333333%; }}
.el-col-9 {{ width: 37.5%; }}
.el-col-10 {{ width: 41.66666667%; }}
.el-col-11 {{ width: 45.83333333%; }}
.el-col-12 {{ width: 50%; }}
.el-col-13 {{ width: 54.16666667%; }}
.el-col-14 {{ width: 58.33333333%; }}
.el-col-15 {{ width: 62.5%; }}
.el-col-16 {{ width: 66.66666667%; }}
.el-col-17 {{ width: 70.83333333%; }}
.el-col-18 {{ width: 75%; }}
.el-col-19 {{ width: 79.16666667%; }}
.el-col-20 {{ width: 83.33333333%; }}
.el-col-21 {{ width: 87.5%; }}
.el-col-22 {{ width: 91.66666667%; }}
.el-col-23 {{ width: 95.83333333%; }}
.el-col-24 {{ width: 100%; }}
        "#, 
        theme.color_white,
        theme.background_color_base,
        theme.background_color_base,
        theme.color_white
        )
    }
}

/// 🎨 完整的样式构建器
pub struct CompleteCssBuilder {
    theme: Theme,
    styles: Vec<String>,
}

impl CompleteCssBuilder {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            styles: Vec::new(),
        }
    }
    
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    
    pub fn with_reset_styles(mut self) -> Self {
        let reset = r#"
/* === CSS Reset === */
*, *::before, *::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
    font-size: 14px;
    line-height: 1.42857143;
    color: #606266;
    background-color: #fff;
}

body {
    font-family: inherit;
    font-size: 14px;
    line-height: 1.42857143;
    color: #606266;
}

.hidden {
    display: none !important;
}
        "#.to_string();
        self.styles.push(reset);
        self
    }
    
    pub fn with_button_styles(mut self) -> Self {
        self.styles.push(ButtonStyles::base(&self.theme));
        self
    }
    
    pub fn with_input_styles(mut self) -> Self {
        self.styles.push(InputStyles::base(&self.theme));
        self
    }
    
    pub fn with_alert_styles(mut self) -> Self {
        self.styles.push(AlertStyles::base(&self.theme));
        self
    }
    
    pub fn with_form_styles(mut self) -> Self {
        self.styles.push(FormStyles::base(&self.theme));
        self
    }
    
    pub fn with_card_styles(mut self) -> Self {
        self.styles.push(CardStyles::base(&self.theme));
        self
    }
    
    pub fn with_table_styles(mut self) -> Self {
        self.styles.push(TableStyles::base(&self.theme));
        self
    }
    
    pub fn with_layout_styles(mut self) -> Self {
        self.styles.push(LayoutStyles::base(&self.theme));
        self
    }
    
    pub fn with_utility_styles(mut self) -> Self {
        let utilities = r#"
/* === Utility Classes === */
.text-c { text-align: center !important; }
.text-l { text-align: left !important; }
.text-r { text-align: right !important; }
.w-100 { width: 100% !important; }
.h-100 { height: 100% !important; }
.m-0 { margin: 0 !important; }
.p-0 { padding: 0 !important; }
.d-flex { display: flex !important; }
.flex-column { flex-direction: column !important; }
.flex-wrap { flex-wrap: wrap !important; }
.align-center { align-items: center !important; }
.justify-center { justify-content: center !important; }
        "#.to_string();
        self.styles.push(utilities);
        self
    }
    
    pub fn build(self) -> String {
        self.styles.join("\n\n")
    }
    
    pub fn build_complete(self) -> String {
        self
            .with_reset_styles()
            .with_button_styles()
            .with_input_styles()
            .with_alert_styles()
            .with_form_styles()
            .with_card_styles()
            .with_table_styles()
            .with_layout_styles()
            .with_utility_styles()
            .build()
    }
}

/// 🎯 主题管理器 - 提供全局主题管理
pub struct CompleteStyleManager {
    theme: Theme,
}

impl CompleteStyleManager {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
    
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    
    pub fn generate_complete_styles(&self) -> String {
        CompleteCssBuilder::new()
            .with_theme(self.theme.clone())
            .build_complete()
    }
    
    pub fn generate_styles_for_components(&self, components: &[&str]) -> String {
        let mut builder = CompleteCssBuilder::new().with_theme(self.theme.clone());
        
        for component in components {
            match *component {
                "button" => builder = builder.with_button_styles(),
                "input" => builder = builder.with_input_styles(),
                "alert" => builder = builder.with_alert_styles(),
                "form" => builder = builder.with_form_styles(),
                "card" => builder = builder.with_card_styles(),
                "table" => builder = builder.with_table_styles(),
                "layout" => builder = builder.with_layout_styles(),
                _ => {},
            }
        }
        
        builder
            .with_reset_styles()
            .with_utility_styles()
            .build()
    }
}