use dioxus::prelude::*;

// Form CSS class constants
pub const FORM: &str = "el-form";
pub const FORM_HORIZONTAL: &str = "el-form--horizontal";
pub const FORM_INLINE: &str = "el-form--inline";

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
///
/// This component provides a styled container for form items with consistent
/// layout and validation state management.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::form::Form;
/// use dioxus_element_plug::components::form_item::FormItem;
/// use dioxus_element_plug::components::input::{Input, InputType};
///
/// rsx! {
///     Form {
///         layout: "horizontal".to_string(),
///         label_position: "right".to_string(),
///         on_submit: move |_| println!("Form submitted"),
///
///         FormItem {
///             label: "Name",
///             Input {
///                 input_type: InputType::Text,
///                 placeholder: "Enter your name",
///             }
///         }
///     }
/// }
/// ```
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
