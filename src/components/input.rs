use dioxus::prelude::*;
use crate::theme::classes;

/// Input sizes matching theme-chalk
#[derive(Clone, PartialEq)]
pub enum InputSize {
    Large,
    Medium,
    Small,
    Mini,
}

impl InputSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            InputSize::Large => classes::INPUT_LARGE,
            InputSize::Medium => classes::INPUT_MEDIUM,
            InputSize::Small => classes::INPUT_SMALL,
            InputSize::Mini => classes::INPUT_MINI,
        }
    }
}

/// Input types
#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Url,
    Number,
    Tel,
    Search,
    Textarea,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Url => "url",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Search => "search",
            InputType::Textarea => "textarea",
        }
    }
}

/// Input props for the theme-chalk styled input component
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Input value
    #[props(default)]
    pub value: Option<String>,

    /// Input placeholder
    #[props(default)]
    pub placeholder: Option<String>,

    /// Input type
    #[props(default = InputType::Text)]
    pub input_type: InputType,

    /// Input size
    #[props(default)]
    pub size: Option<InputSize>,

    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the input is readonly
    #[props(default = false)]
    pub readonly: bool,

    /// Whether to show clear button
    #[props(default = false)]
    pub clearable: bool,

    /// Whether to show password toggle (for password inputs)
    #[props(default = false)]
    pub show_password: bool,

    /// Whether the input has error state
    #[props(default = false)]
    pub error: bool,

    /// Input label
    #[props(default)]
    pub label: Option<String>,

    /// Input prefix icon (CSS class)
    #[props(default)]
    pub prefix_icon: Option<String>,

    /// Input suffix icon (CSS class)
    #[props(default)]
    pub suffix_icon: Option<String>,

    /// Maximum length
    #[props(default)]
    pub maxlength: Option<u32>,

    /// Minimum length
    #[props(default)]
    pub minlength: Option<u32>,

    /// Whether to auto focus
    #[props(default = false)]
    pub autofocus: bool,

    /// Name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Id attribute
    #[props(default)]
    pub id: Option<String>,

    /// Input event handler
    #[props(default)]
    pub on_input: Option<EventHandler<FormEvent>>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,

    /// Focus event handler
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,

    /// Blur event handler
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,

    /// Key down event handler
    #[props(default)]
    pub on_keydown: Option<EventHandler<KeyboardEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// An input component styled with theme-chalk CSS
///
/// This component wraps the native HTML input/textarea element and applies
/// theme-chalk CSS classes for consistent styling.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_theme_chalk::components::input::{Input, InputType, InputSize};
///
/// rsx! {
///     Input {
///         input_type: InputType::Text,
///         placeholder: "Enter your name",
///         size: Some(InputSize::Medium),
///         on_input: move |evt| println!("Input: {}", evt.value()),
///     }
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    let base_class = "el-input";
    let mut wrapper_classes = vec![base_class];

    if let Some(size) = &props.size {
        wrapper_classes.push(size.as_class());
    }

    if props.disabled {
        wrapper_classes.push(classes::IS_DISABLED);
    }

    if props.error {
        wrapper_classes.push("is-error");
    }

    let wrapper_class_string = wrapper_classes.join(" ");

    let input_class = "el-input__inner";

    let input_element = if props.input_type == InputType::Textarea {
        rsx! {
            textarea {
                class: "{input_class}",
                value: props.value.unwrap_or_default(),
                placeholder: props.placeholder.unwrap_or_default(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: props.maxlength.map(|n| n.to_string()).unwrap_or_default(),
                minlength: props.minlength.map(|n| n.to_string()).unwrap_or_default(),
                autofocus: props.autofocus,
                name: props.name.unwrap_or_default(),
                id: props.id.unwrap_or_default(),
                style: props.style.unwrap_or_default(),
                oninput: move |event| {
                    if let Some(handler) = props.on_input {
                        handler.call(event);
                    }
                },
                onchange: move |event| {
                    if let Some(handler) = props.on_change {
                        handler.call(event);
                    }
                },
                onfocus: move |event| {
                    if let Some(handler) = props.on_focus {
                        handler.call(event);
                    }
                },
                onblur: move |event| {
                    if let Some(handler) = props.on_blur {
                        handler.call(event);
                    }
                },
                onkeydown: move |event| {
                    if let Some(handler) = props.on_keydown {
                        handler.call(event);
                    }
                },
            }
        }
    } else {
        rsx! {
            input {
                class: "{input_class}",
                r#type: props.input_type.as_str(),
                value: props.value.unwrap_or_default(),
                placeholder: props.placeholder.unwrap_or_default(),
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: props.maxlength.map(|n| n.to_string()).unwrap_or_default(),
                minlength: props.minlength.map(|n| n.to_string()).unwrap_or_default(),
                autofocus: props.autofocus,
                name: props.name.unwrap_or_default(),
                id: props.id.unwrap_or_default(),
                style: props.style.unwrap_or_default(),
                oninput: move |event| {
                    if let Some(handler) = props.on_input {
                        handler.call(event);
                    }
                },
                onchange: move |event| {
                    if let Some(handler) = props.on_change {
                        handler.call(event);
                    }
                },
                onfocus: move |event| {
                    if let Some(handler) = props.on_focus {
                        handler.call(event);
                    }
                },
                onblur: move |event| {
                    if let Some(handler) = props.on_blur {
                        handler.call(event);
                    }
                },
                onkeydown: move |event| {
                    if let Some(handler) = props.on_keydown {
                        handler.call(event);
                    }
                },
            }
        }
    };

    rsx! {
        div {
            class: "{wrapper_class_string}",

            if let Some(ref label) = props.label {
                label {
                    class: "el-form-item__label",
                    "{label}"
                }
            }

            div {
                class: "el-input__wrapper",

                if let Some(ref prefix_icon) = props.prefix_icon {
                    span {
                        class: "el-input__prefix",
                        i {
                            class: "{prefix_icon} el-input__icon"
                        }
                    }
                }

                {input_element}

                if props.clearable {
                    span {
                        class: "el-input__suffix",
                        i {
                            class: "el-input__icon el-icon-circle-close"
                        }
                    }
                }

                if props.show_password && props.input_type == InputType::Password {
                    span {
                        class: "el-input__suffix",
                        i {
                            class: "el-input__icon el-icon-view"
                        }
                    }
                }

                if let Some(ref suffix_icon) = props.suffix_icon {
                    span {
                        class: "el-input__suffix",
                        i {
                            class: "{suffix_icon} el-input__icon"
                        }
                    }
                }
            }
        }
    }
}

/// Search input with built-in search icon
#[component]
pub fn SearchInput(props: InputProps) -> Element {
    let mut props = props;
    if props.suffix_icon.is_none() {
        props.suffix_icon = Some("el-icon-search".to_string());
    }
    Input(props)
}

/// Password input with toggle visibility
#[component]
pub fn PasswordInput(props: InputProps) -> Element {
    let mut props = props;
    props.input_type = InputType::Password;
    props.show_password = true;
    Input(props)
}
