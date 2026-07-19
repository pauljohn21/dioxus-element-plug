use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// FormItem size variants
#[derive(Clone, PartialEq)]
pub enum FormItemSize {
    Default,
    Large,
    Small,
}

impl FormItemSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            FormItemSize::Default => "",
            FormItemSize::Large => "el-form-item--large",
            FormItemSize::Small => "el-form-item--small",
        }
    }
}

/// FormItem label position
#[derive(Clone, PartialEq)]
pub enum LabelPosition {
    Left,
    Right,
    Top,
}

impl LabelPosition {
    pub fn as_class(&self) -> &'static str {
        match self {
            LabelPosition::Left => "el-form-item--label-left",
            LabelPosition::Right => "el-form-item--label-right",
            LabelPosition::Top => "el-form-item--label-top",
        }
    }
}

/// FormItem props
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    /// Form item content (input, select, etc.)
    #[props(default)]
    pub children: Element,

    /// Field label text
    #[props(default)]
    pub label: Option<String>,

    /// Field prop name (for form validation)
    #[props(default)]
    pub prop: Option<String>,

    /// Label width (e.g., "120px", "auto")
    #[props(default)]
    pub label_width: Option<String>,

    /// Whether the field is required
    #[props(default = false)]
    pub required: bool,

    /// Error message to display
    #[props(default)]
    pub error: Option<String>,

    /// Whether to show error message
    #[props(default = true)]
    pub show_error: bool,

    /// Whether to display error inline
    #[props(default = false)]
    pub inline_message: bool,

    /// Validation status (success, error, validating)
    #[props(default)]
    pub status: Option<String>,

    /// Label position override
    #[props(default)]
    pub label_position: Option<LabelPosition>,

    /// Form item size
    #[props(default = FormItemSize::Default)]
    pub size: FormItemSize,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// FormItem component for form field wrappers
///
/// Provides consistent styling for form fields including labels,
/// required indicators, error messages, and validation states.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::form_item::{FormItem, FormItemSize};
/// use dioxus_element_plug::components::input::Input;
///
/// rsx! {
///     FormItem {
///         label: "Username",
///         required: true,
///         error: Some("Username is required".to_string()),
///         show_error: true,
///         Input {
///             placeholder: Some("Enter username".to_string()),
///         }
///     }
/// }
/// ```
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    // Build main class string
    let class_string = ClassBuilder::new("el-form-item")
        .add_class(props.size.as_class())
        .add_if("is-required", props.required)
        .add_if("is-error", props.error.is_some())
        .add_if("is-success", props.status.as_ref().map(|s| s == "success").unwrap_or(false))
        .add_if("is-validating", props.status.as_ref().map(|s| s == "validating").unwrap_or(false))
        .add_if(props.label_position.as_ref().map(|p| p.as_class()).unwrap_or(""), props.label_position.is_some())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    // Build label style
    let label_style = props
        .label_width
        .as_ref()
        .map(|w| format!("width: {};", w))
        .unwrap_or_default();

    // Determine error class based on inline_message
    let error_class = if props.inline_message {
        "el-form-item__error el-form-item__error--inline"
    } else {
        "el-form-item__error"
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Label section
            if let Some(ref label) = props.label {
                label {
                    class: "el-form-item__label",
                    style: "{label_style}",
                    // Required asterisk
                    if props.required {
                        span {
                            class: "el-form-item__required",
                            "*"
                        }
                    }
                    "{label}"
                }
            }

            // Content section
            div {
                class: "el-form-item__content",
                {props.children}

                // Error message
                if props.show_error {
                    if let Some(ref error) = props.error {
                        div {
                            class: "{error_class}",
                            "{error}"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_item_size_as_class() {
        assert_eq!(FormItemSize::Default.as_class(), "");
        assert_eq!(FormItemSize::Large.as_class(), "el-form-item--large");
        assert_eq!(FormItemSize::Small.as_class(), "el-form-item--small");
    }

    #[test]
    fn test_label_position_as_class() {
        assert_eq!(LabelPosition::Left.as_class(), "el-form-item--label-left");
        assert_eq!(LabelPosition::Right.as_class(), "el-form-item--label-right");
        assert_eq!(LabelPosition::Top.as_class(), "el-form-item--label-top");
    }

    #[test]
    fn test_form_item_class_builder() {
        let class = ClassBuilder::new("el-form-item")
            .add_class("el-form-item--large")
            .add_if("is-required", true)
            .add_if("is-error", false)
            .build();
        assert_eq!(class, "el-form-item el-form-item--large is-required");
    }
}
