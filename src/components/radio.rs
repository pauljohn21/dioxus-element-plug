use dioxus::prelude::*;

/// Radio props - 单选框
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Radio content
    pub children: Element,

    /// Whether checked (controlled)
    #[props(default = false)]
    pub model_value: bool,

    /// Radio value for group
    #[props(default)]
    pub label: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Border
    #[props(default = false)]
    pub border: bool,

    /// Size
    #[props(default)]
    pub size: Option<RadioSize>,

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

/// Radio size
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

/// Radio component
///
/// Mirrors Element Plus `el-radio`.
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut class_names = vec!["el-radio".to_string()];

    if props.model_value {
        class_names.push("is-checked".to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
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
                class: "el-radio__input",

                span {
                    class: if checked { "el-radio__inner is-checked" } else { "el-radio__inner" },
                }

                input {
                    class: "el-radio__original",
                    r#type: "radio",
                    checked: checked,
                    disabled: disabled,
                    onchange: move |_: Event<FormData>| {
                        if !disabled {
                            if let Some(handler) = on_change.as_ref() {
                                handler.call(true);
                            }
                        }
                    },
                }
            }

            span {
                class: "el-radio__label",
                {props.children}
            }
        }
    }
}

/// RadioGroup props
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Group content
    pub children: Element,

    /// Selected value
    #[props(default)]
    pub model_value: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Size
    #[props(default)]
    pub size: Option<RadioSize>,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// RadioGroup component
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let mut class_names = vec!["el-radio-group".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            role: "radiogroup",
            {props.children}
        }
    }
}

/// RadioButton props
#[derive(Props, Clone, PartialEq)]
pub struct RadioButtonProps {
    /// Button content
    pub children: Element,

    /// Whether active
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

/// RadioButton component
#[component]
pub fn RadioButton(props: RadioButtonProps) -> Element {
    let mut class_names = vec!["el-radio-button".to_string()];
    if props.model_value {
        class_names.push("is-active".to_string());
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
                class: "el-radio-button__inner",
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
