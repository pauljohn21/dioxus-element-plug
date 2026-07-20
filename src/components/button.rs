use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

#[cfg(feature = "icons")]
use element_icons::element::Loading;

// Button CSS class constants
pub const BUTTON: &str = "el-button";
pub const BUTTON_PRIMARY: &str = "el-button--primary";
pub const BUTTON_SUCCESS: &str = "el-button--success";
pub const BUTTON_WARNING: &str = "el-button--warning";
pub const BUTTON_DANGER: &str = "el-button--danger";
pub const BUTTON_INFO: &str = "el-button--info";
pub const BUTTON_LARGE: &str = "el-button--large";
pub const BUTTON_SMALL: &str = "el-button--small";
pub const BUTTON_DISABLED: &str = "is-disabled";

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
            ButtonVariant::Default => "",
            ButtonVariant::Primary => BUTTON_PRIMARY,
            ButtonVariant::Success => BUTTON_SUCCESS,
            ButtonVariant::Warning => BUTTON_WARNING,
            ButtonVariant::Danger => BUTTON_DANGER,
            ButtonVariant::Info => BUTTON_INFO,
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
            ButtonSize::Large => "el-button--large",
            ButtonSize::Medium => "el-button--medium",
            ButtonSize::Small => "el-button--small",
            ButtonSize::Mini => "el-button--mini",
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
    let class_string = ClassBuilder::new("el-button")
        .add_class(props.variant.as_class())
        .add_opt_str(props.size.as_ref().map(|s| s.as_class()))
        .add_if("is-disabled", props.disabled)
        .add_if("is-round", props.round)
        .add_if("is-circle", props.circle)
        .add_if("is-loading", props.loading)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_click = props.on_click;

    let mut icon_element = None;
    if let Some(ref icon) = props.icon {
        icon_element = Some(rsx! {
            i {
                class: "{icon}"
            }
        });
    }

    let loading_element = if props.loading {
        Some(render_loading_icon())
    } else {
        None
    };

    rsx! {
        button {
            class: "{class_string}",
            r#type: props.button_type,
            disabled: props.disabled,
            style: "{style_string}",
            onclick: move |event| {
                fire_event(&on_click, event);
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

/// Render loading icon with conditional compilation
#[cfg(feature = "icons")]
fn render_loading_icon() -> Element {
    rsx! {
        Loading {}
    }
}

/// Render loading icon fallback when icons feature is disabled
#[cfg(not(feature = "icons"))]
fn render_loading_icon() -> Element {
    rsx! {
        i {
            class: "el-icon-loading"
        }
    }
}

/// Link button variant
#[component]
pub fn LinkButton(props: ButtonProps) -> Element {
    let on_click = props.on_click;
    rsx! {
        a {
            class: "el-button el-button--text",
            href: "javascript:void(0)",
            onclick: move |event| {
                fire_event(&on_click, event);
            },
            {props.children}
        }
    }
}
