use crate::components::common::{fire_event, style_str, ClassBuilder};
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
    let layout_class = format!("el-form--{}", props.layout);
    let size_class = props.size.as_ref().map(|s| format!("el-form--{}", s));
    let class_string = ClassBuilder::new("el-form")
        .add_class(&layout_class)
        .add_opt_str(size_class.as_deref())
        .add_if("el-form--show-asterisk", props.show_asterisk)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let form_style = format!(
        "--el-form-label-width: {}px; {}",
        props.label_width, style_string
    );

    let on_submit = props.on_submit;

    rsx! {
        form {
            class: "{class_string}",
            style: "{form_style}",
            onsubmit: move |event| {
                event.prevent_default();
                fire_event(&on_submit, event);
            },
            {props.children}
        }
    }
}
