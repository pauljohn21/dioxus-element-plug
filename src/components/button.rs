use dioxus::prelude::*;
use crate::theme::classes;

/// Button variants matching theme-chalk styles
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl ButtonVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "el-button",
            ButtonVariant::Primary => classes::BUTTON_PRIMARY,
            ButtonVariant::Success => classes::BUTTON_SUCCESS,
            ButtonVariant::Warning => classes::BUTTON_WARNING,
            ButtonVariant::Danger => classes::BUTTON_DANGER,
            ButtonVariant::Info => classes::BUTTON_INFO,
        }
    }
}

/// Button sizes matching theme-chalk
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Large,
    Medium,
    Small,
    Mini,
}

impl ButtonSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            ButtonSize::Large => classes::BUTTON_LARGE,
            ButtonSize::Medium => classes::BUTTON_MEDIUM,
            ButtonSize::Small => classes::BUTTON_SMALL,
            ButtonSize::Mini => classes::BUTTON_MINI,
        }
    }
}

/// Button props for the theme-chalk styled button component
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Button content
    #[props(!optional)]
    pub children: Element,

    /// Button variant/style
    #[props(default = ButtonVariant::Default)]
    pub variant: ButtonVariant,

    /// Button size
    #[props(default)]
    pub size: Option<ButtonSize>,

    /// Whether the button is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the button is round
    #[props(default = false)]
    pub round: bool,

    /// Whether the button is circle
    #[props(default = false)]
    pub circle: bool,

    /// Whether the button is loading
    #[props(default = false)]
    pub loading: bool,

    /// Button icon (CSS class name)
    #[props(default)]
    pub icon: Option<String>,

    /// Native button type
    #[props(default = "button".to_string())]
    pub button_type: String,

    /// Click event handler
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A button component styled with theme-chalk CSS
///
/// This component wraps the native HTML button element and applies
/// theme-chalk CSS classes for consistent styling.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_theme_chalk::components::button::{Button, ButtonVariant, ButtonSize};
///
/// rsx! {
///     Button {
///         variant: ButtonVariant::Primary,
///         size: Some(ButtonSize::Medium),
///         on_click: move |_| println!("Button clicked!"),
///         "Click me!"
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_class = "el-button";
    let variant_class = props.variant.as_class();

    let mut classes = vec![base_class, variant_class];

    if let Some(size) = &props.size {
        classes.push(size.as_class());
    }

    if props.disabled {
        classes.push(classes::IS_DISABLED);
    }

    if props.round {
        classes.push("is-round");
    }

    if props.circle {
        classes.push("is-circle");
    }

    if props.loading {
        classes.push("is-loading");
    }

    if let Some(ref custom_class) = props.class {
        classes.push(custom_class);
    }

    let class_string = classes.join(" ");

    let mut icon_element = None;
    if let Some(ref icon) = props.icon {
        icon_element = Some(rsx! {
            i {
                class: "{icon}"
            }
        });
    }

    let mut loading_element = None;
    if props.loading {
        loading_element = Some(rsx! {
            i {
                class: "el-icon-loading"
            }
        });
    }

    rsx! {
        button {
            class: "{class_string}",
            r#type: props.button_type,
            disabled: props.disabled,
            style: props.style.unwrap_or_default(),
            onclick: move |event| {
                if let Some(handler) = props.on_click {
                    handler.call(event);
                }
            },

            {loading_element},
            {icon_element},
            {props.children}
        }
    }
}

/// Outlined button variant
#[component]
pub fn OutlineButton(props: ButtonProps) -> Element {
    let mut props = props;
    let mut class = props.class.unwrap_or_default();
    class.push_str(" is-plain");
    props.class = Some(class);

    Button(props)
}

/// Text button variant
#[component]
pub fn TextButton(props: ButtonProps) -> Element {
    let mut props = props;
    props.variant = ButtonVariant::Default;
    let mut class = props.class.unwrap_or_default();
    class.push_str(" el-button--text");
    props.class = Some(class);

    Button(props)
}

/// Link button variant
#[component]
pub fn LinkButton(props: ButtonProps) -> Element {
    rsx! {
        a {
            class: "el-button el-button--text",
            href: "javascript:void(0)",
            onclick: move |event| {
                if let Some(handler) = props.on_click {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}
