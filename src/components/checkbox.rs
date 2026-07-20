use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// Checkbox size variants
#[derive(Clone, PartialEq)]
pub enum CheckboxSize {
    Large,
    Default,
    Small,
}

impl CheckboxSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            CheckboxSize::Large => "el-checkbox--large",
            CheckboxSize::Default => "",
            CheckboxSize::Small => "el-checkbox--small",
        }
    }
}

/// Checkbox props
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Checkbox label/content
    #[props(default)]
    pub children: Option<Element>,

    /// Label text (used when no children)
    #[props(default)]
    pub label: Option<String>,

    /// Whether the checkbox is checked
    #[props(default = false)]
    pub model_value: bool,

    /// Whether the checkbox is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the checkbox is indeterminate
    #[props(default = false)]
    pub indeterminate: bool,

    /// Checkbox size
    #[props(default = CheckboxSize::Default)]
    pub size: CheckboxSize,

    /// Checkbox value (used in checkbox group)
    #[props(default)]
    pub value: Option<String>,

    /// Checkbox name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Checkbox component for selecting options
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Checkbox {
///         model_value: true,
///         label: Some("Accept terms".to_string()),
///         on_change: move |v| println!("Checked: {}", v),
///     }
/// }
/// ```
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class_string = ClassBuilder::new("el-checkbox")
        .add_class(props.size.as_class())
        .add_if("is-checked", props.model_value)
        .add_if("is-disabled", props.disabled)
        .add_if("is-indeterminate", props.indeterminate)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;

    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            role: "checkbox",
            aria_checked: "{props.model_value}",
            span {
                class: "el-checkbox__input",
                if props.model_value || props.indeterminate {
                    span {
                        class: "el-checkbox__inner",
                        if props.indeterminate {
                            i { class: "el-checkbox__indeterminate" }
                        }
                    }
                } else {
                    span { class: "el-checkbox__inner" }
                }
                if let Some(ref name) = props.name {
                    input {
                        r#type: "checkbox",
                        class: "el-checkbox__original",
                        name: "{name}",
                        checked: props.model_value,
                        onchange: move |_| {
                            if !props.disabled {
                                fire_event(&on_change, !props.model_value);
                            }
                        },
                    }
                }
            }
            span {
                class: "el-checkbox__label",
                onclick: move |_| {
                    if !props.disabled {
                        fire_event(&on_change, !props.model_value);
                    }
                },
                if let Some(ref label) = props.label {
                    "{label}"
                }
                {props.children}
            }
        }
    }
}

/// CheckboxGroup props
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupProps {
    /// Checkbox group content
    #[props(default)]
    pub children: Element,

    /// Whether the group is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Minimum checked items
    #[props(default)]
    pub min: Option<u32>,

    /// Maximum checked items
    #[props(default)]
    pub max: Option<u32>,

    /// Group name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// CheckboxGroup component for grouping multiple checkboxes
#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
    let class_string = ClassBuilder::new("el-checkbox-group")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "group",
            {props.children}
        }
    }
}

/// CheckboxButton props (same as Checkbox but with button styling)
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxButtonProps {
    /// Checkbox button content
    #[props(default)]
    pub children: Option<Element>,

    /// Label text
    #[props(default)]
    pub label: Option<String>,

    /// Whether checked
    #[props(default = false)]
    pub model_value: bool,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// CheckboxButton component - checkbox styled as a button
#[component]
pub fn CheckboxButton(props: CheckboxButtonProps) -> Element {
    let class_string = ClassBuilder::new("el-checkbox-button")
        .add_if("is-checked", props.model_value)
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;

    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            span {
                class: "el-checkbox-button__inner",
                onclick: move |_| {
                    if !props.disabled {
                        fire_event(&on_change, !props.model_value);
                    }
                },
                if let Some(ref label) = props.label {
                    "{label}"
                }
                {props.children}
            }
        }
    }
}
