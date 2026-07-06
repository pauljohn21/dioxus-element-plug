use dioxus::prelude::*;

/// Checkbox props - 多选框
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Checkbox label/content
    pub children: Element,

    /// Whether checked (controlled)
    #[props(default = false)]
    pub model_value: bool,

    /// Checkbox value for group
    #[props(default)]
    pub label: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether indeterminate
    #[props(default = false)]
    pub indeterminate: bool,

    /// Border
    #[props(default = false)]
    pub border: bool,

    /// Size
    #[props(default)]
    pub size: Option<CheckboxSize>,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Checkbox size
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

/// Checkbox component
///
/// Mirrors Element Plus `el-checkbox`.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut class_names = vec!["el-checkbox".to_string()];

    if props.model_value {
        class_names.push("is-checked".to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if props.indeterminate {
        class_names.push("is-indeterminate".to_string());
    }
    if props.border {
        class_names.push("is-bordered".to_string());
    }
    if let Some(ref s) = props.size {
        class_names.push(s.as_class().to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let on_change = props.on_change;
    let checked = props.model_value;
    let disabled = props.disabled;

    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",

            span {
                class: "el-checkbox__input",

                span {
                    class: if checked { "el-checkbox__inner is-checked" } else { "el-checkbox__inner" },
                }

                input {
                    class: "el-checkbox__original",
                    r#type: "checkbox",
                    checked: checked,
                    disabled: disabled,
                    onchange: move |_e: Event<FormData>| {
                        if !disabled {
                            if let Some(handler) = on_change.as_ref() {
                                handler.call(!checked);
                            }
                        }
                    },
                }
            }

            span {
                class: "el-checkbox__label",
                {props.children}
            }
        }
    }
}

/// CheckboxGroup props
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupProps {
    /// Group content
    pub children: Element,

    /// Selected values
    #[props(default)]
    pub model_value: Vec<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// CheckboxGroup component
#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
    let mut class_names = vec!["el-checkbox-group".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            role: "group",
            {props.children}
        }
    }
}

/// CheckboxButton props
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxButtonProps {
    /// Button content
    pub children: Element,

    /// Whether checked
    #[props(default = false)]
    pub model_value: bool,

    /// Label value
    #[props(default)]
    pub label: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// CheckboxButton component
#[component]
pub fn CheckboxButton(props: CheckboxButtonProps) -> Element {
    let mut class_names = vec!["el-checkbox-button".to_string()];
    if props.model_value {
        class_names.push("is-checked".to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let on_change = props.on_change;
    let checked = props.model_value;
    let disabled = props.disabled;

    rsx! {
        label {
            class: "{class_string}",

            span {
                class: "el-checkbox-button__inner",
                onclick: move |_| {
                    if !disabled {
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(!checked);
                        }
                    }
                },
                {props.children}
            }
        }
    }
}
