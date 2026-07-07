use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

// Input CSS class constants
pub const INPUT: &str = "el-input";
pub const INPUT_INNER: &str = "el-input__inner";
pub const INPUT_WRAPPER: &str = "el-input__wrapper";
pub const INPUT_PREFIX: &str = "el-input__prefix";
pub const INPUT_SUFFIX: &str = "el-input__suffix";
pub const INPUT_DISABLED: &str = "is-disabled";
pub const INPUT_LARGE: &str = "el-input--large";
pub const INPUT_MEDIUM: &str = "el-input--medium";
pub const INPUT_SMALL: &str = "el-input--small";
pub const INPUT_MINI: &str = "el-input--mini";
pub const INPUT_ERROR: &str = "is-error";
pub const INPUT_GROUP: &str = "el-input-group";
pub const INPUT_GROUP_APPEND: &str = "el-input-group__append";
pub const INPUT_GROUP_PREPEND: &str = "el-input-group__prepend";

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
            InputSize::Large => "el-input--large",
            InputSize::Medium => "el-input--medium",
            InputSize::Small => "el-input--small",
            InputSize::Mini => "el-input--mini",
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
/// use dioxus_element_plug::components::input::{Input, InputType, InputSize};
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
    let mut focused = use_signal(|| false);

    let mut outer_builder = ClassBuilder::new(INPUT);

    if let Some(ref size) = props.size {
        outer_builder = outer_builder.add_class(size.as_class());
    }

    let outer_class = outer_builder
        .add_if("is-disabled", props.disabled)
        .add_if("is-error", props.error)
        .add_opt(props.class.as_ref())
        .build();

    let wrapper_class = ClassBuilder::new(INPUT_WRAPPER)
        .add_if("is-focus", focused())
        .add_if("is-disabled", props.disabled)
        .build();

    let inner_style = style_str(&props.style);
    let value = props.value.clone().unwrap_or_default();
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let name = props.name.clone().unwrap_or_default();
    let id = props.id.clone().unwrap_or_default();
    let maxlength = props.maxlength.map(|n| n.to_string()).unwrap_or_default();
    let minlength = props.minlength.map(|n| n.to_string()).unwrap_or_default();

    let on_input = props.on_input;
    let on_change = props.on_change;
    let on_focus = props.on_focus;
    let on_blur = props.on_blur;
    let on_keydown = props.on_keydown;

    let input_element = if props.input_type == InputType::Textarea {
        rsx! {
            textarea {
                class: INPUT_INNER,
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: "{maxlength}",
                minlength: "{minlength}",
                autofocus: props.autofocus,
                name: "{name}",
                id: "{id}",
                style: "{inner_style}",
                oninput: move |event| fire_event(&on_input, event),
                onchange: move |event| fire_event(&on_change, event),
                onfocus: move |event| {
                    focused.set(true);
                    fire_event(&on_focus, event);
                },
                onblur: move |event| {
                    focused.set(false);
                    fire_event(&on_blur, event);
                },
                onkeydown: move |event| fire_event(&on_keydown, event),
            }
        }
    } else {
        rsx! {
            input {
                class: INPUT_INNER,
                r#type: props.input_type.as_str(),
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: props.disabled,
                readonly: props.readonly,
                maxlength: "{maxlength}",
                minlength: "{minlength}",
                autofocus: props.autofocus,
                name: "{name}",
                id: "{id}",
                style: "{inner_style}",
                oninput: move |event| fire_event(&on_input, event),
                onchange: move |event| fire_event(&on_change, event),
                onfocus: move |event| {
                    focused.set(true);
                    fire_event(&on_focus, event);
                },
                onblur: move |event| {
                    focused.set(false);
                    fire_event(&on_blur, event);
                },
                onkeydown: move |event| fire_event(&on_keydown, event),
            }
        }
    };

    rsx! {
        div {
            class: "{outer_class}",

            if let Some(ref label) = props.label {
                label {
                    class: "el-form-item__label",
                    "{label}"
                }
            }

            div {
                class: "{wrapper_class}",

                if let Some(ref prefix_icon) = props.prefix_icon {
                    span {
                        class: INPUT_PREFIX,
                        i { class: "{prefix_icon} el-input__icon" }
                    }
                }

                {input_element}

                if props.clearable {
                    span {
                        class: INPUT_SUFFIX,
                        i { class: "el-input__icon el-icon-circle-close" }
                    }
                }

                if props.show_password && props.input_type == InputType::Password {
                    span {
                        class: INPUT_SUFFIX,
                        i { class: "el-input__icon el-icon-view" }
                    }
                }

                if let Some(ref suffix_icon) = props.suffix_icon {
                    span {
                        class: INPUT_SUFFIX,
                        i { class: "{suffix_icon} el-input__icon" }
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
