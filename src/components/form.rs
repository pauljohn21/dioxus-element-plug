use dioxus::prelude::*;

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
