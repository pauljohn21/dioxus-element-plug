use dioxus::prelude::*;

/// FormItem props
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    #[props(default)]
    pub children: Element,

    /// Field label
    #[props(default)]
    pub label: Option<String>,

    /// Field prop name
    #[props(default)]
    pub prop: Option<String>,

    /// Field width
    #[props(default)]
    pub label_width: Option<String>,

    /// Whether the field is required
    #[props(default = false)]
    pub required: bool,

    /// Error message
    #[props(default)]
    pub error: Option<String>,

    /// Whether to show error
    #[props(default = false)]
    pub show_error: bool,

    /// Inline message
    #[props(default = false)]
    pub inline_message: bool,

    /// Validation status
    #[props(default)]
    pub status: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// FormItem component for form field wrappers
#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    let mut class_names = vec!["el-form-item".to_string()];
    if props.required { class_names.push("is-required".to_string()); }
    if props.error.is_some() { class_names.push("is-error".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let label_style = props.label_width.clone().map(|w| format!("width: {};", w)).unwrap_or_default();

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            if let Some(ref label) = props.label {
                label {
                    class: "el-form-item__label",
                    style: "{label_style}",
                    "{label}"
                }
            }
            div {
                class: "el-form-item__content",
                {props.children}
                if let Some(ref error) = props.error {
                    div {
                        class: "el-form-item__error",
                        "{error}"
                    }
                }
            }
        }
    }
}
