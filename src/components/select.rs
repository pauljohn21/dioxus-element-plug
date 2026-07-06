use dioxus::prelude::*;

/// Select option data
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self { value: value.to_string(), label: label.to_string(), disabled: false }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Select size
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

/// Select props - 选择器
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Selected value
    #[props(default)]
    pub model_value: Option<String>,

    /// Options
    #[props(default)]
    pub options: Vec<SelectOption>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether clearable
    #[props(default = false)]
    pub clearable: bool,

    /// Whether filterable
    #[props(default = false)]
    pub filterable: bool,

    /// Whether multiple
    #[props(default = false)]
    pub multiple: bool,

    /// Placeholder
    #[props(default = "Select".to_string())]
    pub placeholder: String,

    /// Size
    #[props(default)]
    pub size: Option<SelectSize>,

    /// Collapse tags
    #[props(default)]
    pub collapse_tags: Option<bool>,

    /// Collapse tags tooltip
    #[props(default)]
    pub collapse_tags_tooltip: Option<bool>,

    /// Max collapse tags
    #[props(default)]
    pub max_collapse_tags: Option<u32>,

    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Select component
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut class_names = vec!["el-select".to_string()];
    if let Some(ref s) = props.size {
        class_names.push(s.as_class().to_string());
    }
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let selected_label = props.model_value.as_ref().and_then(|val| {
        props.options.iter().find(|o| &o.value == val).map(|o| o.label.clone())
    }).unwrap_or_default();

    let on_change = props.on_change;
    let disabled = props.disabled;
    let model_value = props.model_value.clone();

    // Pre-extract option data to avoid borrow issues in rsx!
    let option_data: Vec<(String, String, bool)> = props.options.iter()
        .map(|o| (o.value.clone(), o.label.clone(), o.disabled))
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            div {
                class: "el-select__wrapper",

                div {
                    class: "el-select__input-wrapper",

                    if !selected_label.is_empty() {
                        span { class: "el-select__selected-item", "{selected_label}" }
                    } else {
                        span { class: "el-select__placeholder", "{props.placeholder}" }
                    }
                }
            }

            div {
                class: "el-select__dropdown",

                for (value, label, opt_disabled) in option_data.into_iter() {
                    div {
                        class: if model_value.as_ref() == Some(&value) {
                            "el-select-dropdown__item selected"
                        } else if opt_disabled {
                            "el-select-dropdown__item is-disabled"
                        } else {
                            "el-select-dropdown__item"
                        },
                        onclick: move |_| {
                            if !disabled && !opt_disabled {
                                if let Some(handler) = on_change.as_ref() {
                                    handler.call(value.clone());
                                }
                            }
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}
