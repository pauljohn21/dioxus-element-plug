use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Select option data
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Select size variants
#[derive(Clone, PartialEq)]
pub enum SelectSize {
    Large,
    Default,
    Small,
}

impl SelectSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            SelectSize::Large => "el-select--large",
            SelectSize::Default => "",
            SelectSize::Small => "el-select--small",
        }
    }
}

/// Select props
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Selected value
    #[props(default)]
    pub model_value: Option<String>,

    /// Options for the select
    #[props(default)]
    pub options: Vec<SelectOption>,

    /// Whether the select is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the select is clearable
    #[props(default = false)]
    pub clearable: bool,

    /// Whether the select is filterable
    #[props(default = false)]
    pub filterable: bool,

    /// Whether multiple selection is allowed
    #[props(default = false)]
    pub multiple: bool,

    /// Placeholder text
    #[props(default = "Select".to_string())]
    pub placeholder: String,

    /// Select size
    #[props(default = SelectSize::Default)]
    pub size: SelectSize,

    /// Whether to collapse tags in multiple mode
    #[props(default = false)]
    pub collapse_tags: bool,

    /// Whether to collapse tags with tooltip
    #[props(default)]
    pub collapse_tags_tooltip: Option<bool>,

    /// Maximum number of tags shown in multiple mode
    #[props(default)]
    pub max_collapse_tags: Option<u32>,

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

/// Select component for choosing from a dropdown list
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Select {
///         model_value: Some("1".to_string()),
///         options: vec![
///             SelectOption::new("1", "Option 1"),
///             SelectOption::new("2", "Option 2"),
///         ],
///         on_change: move |v| println!("Selected: {}", v),
///     }
/// }
/// ```
#[component]
pub fn Select(props: SelectProps) -> Element {
    let class_string = ClassBuilder::new("el-select")
        .add_class(props.size.as_class())
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    // Find selected label
    let selected_label = props
        .options
        .iter()
        .find(|opt| Some(&opt.value) == props.model_value.as_ref())
        .map(|opt| opt.label.clone())
        .unwrap_or_default();

    let is_empty = props.options.is_empty();
    let model_value_clone = props.model_value.clone();
    let option_data: Vec<(String, String, bool, String)> = props.options.iter().map(|opt| {
        let is_selected = Some(&opt.value) == model_value_clone.as_ref();
        let cls = ClassBuilder::new("el-select-dropdown__item")
            .add_if("selected", is_selected)
            .add_if("is-disabled", opt.disabled)
            .build();
        (opt.value.clone(), opt.label.clone(), opt.disabled, cls)
    }).collect();

    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-select__wrapper",
                onclick: move |_| {
                    // Toggle dropdown - in a real implementation this would use signals
                },
                span {
                    class: "el-select__selected-item",
                    if selected_label.is_empty() {
                        "{props.placeholder}"
                    } else {
                        "{selected_label}"
                    }
                }
                if props.clearable && props.model_value.is_some() {
                    span {
                        class: "el-select__caret el-select__close",
                        "×"
                    }
                }
                span {
                    class: "el-select__caret",
                    i { class: "el-icon-arrow-down" }
                }
            }
            div {
                class: "el-select__dropdown el-popper",
                style: "display: none;",
                div {
                    class: "el-select-dropdown",
                    if is_empty {
                        p {
                            class: "el-select-dropdown__empty",
                            "No data"
                        }
                    }
                    for (opt_value, opt_label, opt_disabled, opt_class) in option_data.into_iter() {
                        div {
                            class: "{opt_class}",
                            onclick: move |_| {
                                if !opt_disabled {
                                    fire_event(&on_change, opt_value.clone());
                                }
                            },
                            "{opt_label}"
                        }
                    }
                }
            }
        }
    }
}
