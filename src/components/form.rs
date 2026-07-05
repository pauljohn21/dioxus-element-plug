use dioxus::prelude::*;
use crate::components::input::{Input, InputProps, InputType};

// Form CSS class constants
pub const FORM: &str = "el-form";
pub const FORM_HORIZONTAL: &str = "el-form--horizontal";
pub const FORM_INLINE: &str = "el-form--inline";
pub const FORM_ITEM: &str = "el-form-item";
pub const FORM_ITEM_LABEL: &str = "el-form-item__label";
pub const FORM_ITEM_CONTENT: &str = "el-form-item__content";
pub const FORM_ITEM_ERROR: &str = "el-form-item__error";
pub const FORM_ITEM_HELP: &str = "el-form-item__help";
pub const FORM_ITEM_REQUIRED: &str = "is-required";
pub const FORM_ITEM_ERROR_STATE: &str = "is-error";

// Form controls CSS class constants
pub const SELECT: &str = "el-select";
pub const CHECKBOX: &str = "el-checkbox";
pub const RADIO: &str = "el-radio";
pub const SLIDER: &str = "el-slider";
pub const IS_DISABLED: &str = "is-disabled";

/// Form item validation status
#[derive(Clone, PartialEq)]
pub enum FormItemStatus {
    Valid,
    Invalid,
    Validating,
    None,
}

impl FormItemStatus {
    pub fn as_class(&self) -> &'static str {
        match self {
            FormItemStatus::Valid => "is-success",
            FormItemStatus::Invalid => "is-error",
            FormItemStatus::Validating => "is-validating",
            FormItemStatus::None => "",
        }
    }
}

/// Form item props
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    /// Form item label
    pub label: String,

    /// Form item content
    pub children: Element,

    /// Whether the form item is required
    #[props(default = false)]
    pub required: bool,

    /// Form item validation status
    #[props(default = FormItemStatus::None)]
    pub status: FormItemStatus,

    /// Form item error message
    #[props(default)]
    pub error_message: Option<String>,

    /// Form item help text
    #[props(default)]
    pub help_text: Option<String>,

    /// Label width in pixels
    #[props(default = 120)]
    pub label_width: u32,

    /// Additional CSS classes for the form item
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles for the form item
    #[props(default)]
    pub style: Option<String>,
}

/// A form item component with validation support
///
/// This component provides a labeled container for form controls with built-in
/// validation state display and error messaging.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::form::{FormItem, FormItemStatus};
/// use dioxus_element_plug::components::input::{Input, InputType};
///
/// rsx! {
///     FormItem {
///         label: "Email",
///         required: true,
///         status: FormItemStatus::Invalid,
///         error_message: Some("Please enter a valid email address".to_string()),
///         Input {
///             input_type: InputType::Email,
///             placeholder: "Enter your email",
///         }
///     }
/// }
/// ```
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    let mut class_names = vec![FORM_ITEM.to_string()];
    
    if !props.status.as_class().is_empty() {
        class_names.push(props.status.as_class().to_string());
    }
    
    if props.required {
        class_names.push("is-required".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let label_style = format!("width: {}px; display: inline-block;", props.label_width);
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            
            if !props.label.is_empty() {
                label {
                    class: "el-form-item__label",
                    style: "{label_style}",
                    "{props.label}"
                }
            }
            
            div {
                class: "el-form-item__content",
                
                {props.children}
                
                if let Some(ref error_msg) = props.error_message {
                    div {
                        class: "el-form-item__error",
                        "{error_msg}"
                    }
                }
                
                if let Some(ref help_text) = props.help_text {
                    div {
                        class: "el-form-item__help",
                        "{help_text}"
                    }
                }
            }
        }
    }
}

/// Form props
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    /// Form content
    pub children: Element,

    /// Form layout direction
    #[props(default = "horizontal".to_string())]
    pub layout: String,

    /// Label position
    #[props(default = "right".to_string())]
    pub label_position: String,

    /// Label width in pixels
    #[props(default = 120)]
    pub label_width: u32,

    /// Form size
    #[props(default)]
    pub size: Option<String>,

    /// Whether to show required asterisk
    #[props(default = true)]
    pub show_asterisk: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,

    /// Form submit handler
    #[props(default)]
    pub on_submit: Option<EventHandler<FormEvent>>,
}

/// A form component wrapper
///
/// This component provides a styled container for form items with consistent
/// layout and validation state management.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::form::{Form, FormItem, FormItemStatus};
/// use dioxus_element_plug::components::input::{Input, InputType};
///
/// rsx! {
///     Form {
///         layout: "horizontal".to_string(),
///         label_position: "right".to_string(),
///         on_submit: move |_| println!("Form submitted"),
///         
///         FormItem {
///             label: "Name",
///             Input {
///                 input_type: InputType::Text,
///                 placeholder: "Enter your name",
///             }
///         }
///         
///         FormItem {
///             label: "Email",
///             Input {
///                 input_type: InputType::Email,
///                 placeholder: "Enter your email",
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Form(props: FormProps) -> Element {
    let mut class_names = vec![FORM.to_string()];
    
    class_names.push(format!("el-form--{}", props.layout));
    
    if let Some(ref size) = props.size {
        class_names.push(format!("el-form--{}", size));
    }
    
    if props.show_asterisk {
        class_names.push("el-form--show-asterisk".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let form_style = format!(
        "--el-form-label-width: {}px; {}", 
        props.label_width,
        style_string
    );
    
    rsx! {
        form {
            class: "{class_string}",
            style: "{form_style}",
            onsubmit: move |event| {
                event.prevent_default();
                if let Some(handler) = props.on_submit {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}

/// Select option for dropdown
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Select input component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Selected value
    #[props(default)]
    pub value: Option<String>,

    /// Select options
    pub options: Vec<SelectOption>,

    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,

    /// Whether the select is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to allow clearing selection
    #[props(default = false)]
    pub clearable: bool,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A select dropdown component
///
/// This component provides a styled dropdown select with option support
/// and validation capabilities.
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut class_names = vec![SELECT.to_string()];
    
    if props.disabled {
        class_names.push(IS_DISABLED.to_string());
    }
    
    if props.clearable {
        class_names.push("is-clearable".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let selected_option = props.options.iter()
        .find(|opt| Some(opt.value.clone()) == props.value)
        .map(|opt| opt.label.clone())
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            
            div {
                class: "el-select__wrapper",
                
                div {
                    class: "el-select__selection",
                    
                    if selected_option.is_empty() {
                        if let Some(ref placeholder) = props.placeholder {
                            span {
                                class: "el-select__placeholder",
                                "{placeholder}"
                            }
                        }
                    } else {
                        span {
                            class: "el-select__selected-item",
                            "{selected_option}"
                        }
                    }
                    
                    if props.clearable && !selected_option.is_empty() {
                        span {
                            class: "el-select__clear",
                            i {
                                class: "el-icon-close"
                            }
                        }
                    }
                }
                
                i {
                    class: "el-select__caret el-icon-arrow-down"
                }
            }
            
            select {
                class: "el-select__input",
                disabled: props.disabled,
                value: props.value.unwrap_or_default(),
                onchange: move |event| {
                    if let Some(handler) = props.on_change {
                        handler.call(event);
                    }
                },
                
                for option in props.options.iter() {
                    option {
                        value: "{option.value}",
                        disabled: option.disabled,
                        "{option.label}"
                    }
                }
            }
        }
    }
}

/// Checkbox component
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Checkbox label
    #[props(default)]
    pub label: Option<String>,

    /// Whether the checkbox is checked
    #[props(default = false)]
    pub checked: bool,

    /// Whether the checkbox is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A checkbox component
///
/// This component provides a styled checkbox with label support.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut class_names = vec![CHECKBOX.to_string()];
    
    if props.checked {
        class_names.push("is-checked".to_string());
    }
    
    if props.disabled {
        class_names.push(IS_DISABLED.to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            
            span {
                class: "el-checkbox__input",
                
                input {
                    r#type: "checkbox",
                    checked: props.checked,
                    disabled: props.disabled,
                    onchange: move |event| {
                        if let Some(handler) = props.on_change {
                            handler.call(event);
                        }
                    },
                }
                
                span {
                    class: "el-checkbox__inner"
                }
            }
            
            if let Some(ref label_text) = props.label {
                span {
                    class: "el-checkbox__label",
                    "{label_text}"
                }
            }
        }
    }
}

/// Radio button component
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Radio button label
    pub label: String,

    /// Radio value
    pub value: String,

    /// Whether the radio is checked
    #[props(default = false)]
    pub checked: bool,

    /// Whether the radio is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Group name
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A radio button component
///
/// This component provides a styled radio button with label support.
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut class_names = vec![RADIO.to_string()];
    
    if props.checked {
        class_names.push("is-checked".to_string());
    }
    
    if props.disabled {
        class_names.push(IS_DISABLED.to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            
            span {
                class: "el-radio__input",
                
                input {
                    r#type: "radio",
                    name: props.name.unwrap_or_default(),
                    value: props.value,
                    checked: props.checked,
                    disabled: props.disabled,
                    onchange: move |event| {
                        if let Some(handler) = props.on_change {
                            handler.call(event);
                        }
                    },
                }
                
                span {
                    class: "el-radio__inner"
                }
            }
            
            span {
                class: "el-radio__label",
                "{props.label}"
            }
        }
    }
}

/// Slider component for range input
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Current value
    #[props(default = 0.0)]
    pub value: f64,

    /// Minimum value
    #[props(default = 0.0)]
    pub min: f64,

    /// Maximum value
    #[props(default = 100.0)]
    pub max: f64,

    /// Step size
    #[props(default = 1.0)]
    pub step: f64,

    /// Whether the slider is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to show tooltip
    #[props(default = true)]
    pub show_tooltip: bool,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,

    /// Input event handler
    #[props(default)]
    pub on_input: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A slider component for range input
///
/// This component provides a styled range slider with tooltip support.
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut class_names = vec![SLIDER.to_string()];
    
    if props.disabled {
        class_names.push(IS_DISABLED.to_string());
    }
    
    if props.show_tooltip {
        class_names.push("show-tooltip".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let percentage = if props.max > props.min {
        ((props.value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            
            div {
                class: "el-slider__runway",
                
                div {
                    class: "el-slider__bar",
                    style: "width: {percentage}%"
                }
                
                div {
                    class: "el-slider__button-wrapper",
                    style: "left: {percentage}%",
                    
                    div {
                        class: "el-slider__button"
                    }
                    
                    if props.show_tooltip {
                        div {
                            class: "el-tooltip__popper is-dark",
                            "{props.value as i32}"
                        }
                    }
                }
            }
            
            input {
                r#type: "range",
                min: props.min,
                max: props.max,
                step: props.step,
                value: props.value,
                disabled: props.disabled,
                onchange: move |event| {
                    if let Some(handler) = props.on_change {
                        handler.call(event);
                    }
                },
                oninput: move |event| {
                    if let Some(handler) = props.on_input {
                        handler.call(event);
                    }
                },
            }
        }
    }
}