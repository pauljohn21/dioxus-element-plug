use dioxus::prelude::*;

/// Radio size variants
#[derive(Clone, PartialEq)]
pub enum RadioSize {
    Large,
    Default,
    Small,
}

impl RadioSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            RadioSize::Large => "el-radio--large",
            RadioSize::Default => "",
            RadioSize::Small => "el-radio--small",
        }
    }
}

/// Radio props
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Radio content
    #[props(default)]
    pub children: Option<Element>,

    /// Label text
    #[props(default)]
    pub label: Option<String>,

    /// Radio value
    pub value: String,

    /// Whether this radio is checked
    #[props(default = false)]
    pub model_value: bool,

    /// Whether the radio is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to show a border
    #[props(default = false)]
    pub border: bool,

    /// Radio size
    #[props(default = RadioSize::Default)]
    pub size: RadioSize,

    /// Radio name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Radio component for selecting a single option
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Radio { value: "1".to_string(), label: Some("Option 1".to_string()), model_value: true }
/// }
/// ```
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut class_names = vec!["el-radio".to_string()];

    let size_class = props.size.as_class();
    if !size_class.is_empty() {
        class_names.push(size_class.to_string());
    }

    if props.model_value {
        class_names.push("is-checked".to_string());
    }

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if props.border {
        class_names.push("is-bordered".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();
    let value_clone1 = props.value.clone();
    let value_clone2 = props.value.clone();

    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            role: "radio",
            aria_checked: "{props.model_value}",
            span {
                class: "el-radio__input",
                span {
                    class: "el-radio__inner",
                }
                input {
                    r#type: "radio",
                    class: "el-radio__original",
                    name: props.name.clone().unwrap_or_default(),
                    value: "{props.value}",
                    checked: props.model_value,
                    onchange: move |_| {
                        if !props.disabled {
                            if let Some(handler) = props.on_change {
                                handler.call(value_clone1.clone());
                            }
                        }
                    },
                }
            }
            span {
                class: "el-radio__label",
                onclick: move |_| {
                    if !props.disabled {
                        if let Some(handler) = props.on_change {
                            handler.call(value_clone2.clone());
                        }
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

/// RadioGroup props
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Radio group content
    #[props(default)]
    pub children: Element,

    /// Whether the group is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Group size
    #[props(default = RadioSize::Default)]
    pub size: RadioSize,

    /// Text color for checked radio
    #[props(default)]
    pub text_color: Option<String>,

    /// Fill color for checked radio
    #[props(default)]
    pub fill: Option<String>,

    /// Group name attribute
    #[props(default)]
    pub name: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// RadioGroup component for grouping radio buttons
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let mut class_names = vec!["el-radio-group".to_string()];

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "radiogroup",
            {props.children}
        }
    }
}

/// RadioButton props (radio styled as a button)
#[derive(Props, Clone, PartialEq)]
pub struct RadioButtonProps {
    /// Radio button content
    #[props(default)]
    pub children: Option<Element>,

    /// Label text
    #[props(default)]
    pub label: Option<String>,

    /// Radio value
    pub value: String,

    /// Whether this radio is checked
    #[props(default = false)]
    pub model_value: bool,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// RadioButton component - radio styled as a button
#[component]
pub fn RadioButton(props: RadioButtonProps) -> Element {
    let mut class_names = vec!["el-radio-button".to_string()];

    if props.model_value {
        class_names.push("is-checked".to_string());
    }

    if props.disabled {
        class_names.push("is-disabled".to_string());
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();
    let value_clone = props.value.clone();

    rsx! {
        label {
            class: "{class_string}",
            style: "{style_string}",
            span {
                class: "el-radio-button__inner",
                role: "radio",
                aria_checked: "{props.model_value}",
                onclick: move |_| {
                    if !props.disabled {
                        if let Some(handler) = props.on_change {
                            handler.call(value_clone.clone());
                        }
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
