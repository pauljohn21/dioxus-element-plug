use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Autocomplete size variants
#[derive(Clone, PartialEq, Default)]
pub enum AutocompleteSize {
    #[default]
    Default,
    Large,
    Small,
}

impl AutocompleteSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            AutocompleteSize::Default => "",
            AutocompleteSize::Large => "el-autocomplete--large",
            AutocompleteSize::Small => "el-autocomplete--small",
        }
    }
}

/// Autocomplete suggestion item
#[derive(Clone, PartialEq, Debug)]
pub struct AutocompleteSuggestion {
    pub value: String,
    pub label: Option<String>,
}

impl AutocompleteSuggestion {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Autocomplete props
#[derive(Props, Clone, PartialEq)]
pub struct AutocompleteProps {
    /// Input value
    #[props(default)]
    pub model_value: Option<String>,

    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,

    /// Suggestions list
    #[props(default)]
    pub suggestions: Vec<AutocompleteSuggestion>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether clearable
    #[props(default = false)]
    pub clearable: bool,

    /// Whether to trigger on focus
    #[props(default = false)]
    pub trigger_on_focus: bool,

    /// Size variant
    #[props(default = AutocompleteSize::Default)]
    pub size: AutocompleteSize,

    /// Debounce delay in milliseconds
    #[props(default = 300)]
    pub debounce: u32,

    /// Prefix icon class
    #[props(default)]
    pub prefix_icon: Option<String>,

    /// Suffix icon class
    #[props(default)]
    pub suffix_icon: Option<String>,

    /// Input event handler (called on every keystroke)
    #[props(default)]
    pub on_input: Option<EventHandler<String>>,

    /// Select event handler (called when suggestion is selected)
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,

    /// Change event handler (called on blur or enter)
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Focus event handler
    #[props(default)]
    pub on_focus: Option<EventHandler<()>>,

    /// Blur event handler
    #[props(default)]
    pub on_blur: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Autocomplete component for input with suggestions
///
/// Provides an input field with dropdown suggestions that can be filtered.
/// Supports keyboard navigation and selection.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::autocomplete::{Autocomplete, AutocompleteSuggestion};
///
/// let suggestions = vec![
///     AutocompleteSuggestion::new("apple"),
///     AutocompleteSuggestion::new("banana"),
///     AutocompleteSuggestion::new("cherry"),
/// ];
///
/// rsx! {
///     Autocomplete {
///         placeholder: Some("Search fruits...".to_string()),
///         suggestions: suggestions,
///         on_input: move |value: String| println!("Input: {}", value),
///         on_select: move |value: String| println!("Selected: {}", value),
///     }
/// }
/// ```
#[component]
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    let mut panel_visible = use_signal(|| false);
    let mut input_value = use_signal(|| props.model_value.clone().unwrap_or_default());
    let mut highlighted_index = use_signal(|| 0usize);
    let mut is_focused = use_signal(|| false);

    // Update input_value when model_value changes
    use_effect(move || {
        if let Some(ref value) = props.model_value {
            input_value.set(value.clone());
        }
    });

    let placeholder = props.placeholder.clone().unwrap_or_default();

    // Filter suggestions based on input
    let filtered_suggestions: Vec<AutocompleteSuggestion> = if input_value().is_empty() {
        if props.trigger_on_focus || is_focused() {
            props.suggestions.clone()
        } else {
            vec![]
        }
    } else {
        let input_lower = input_value().to_lowercase();
        props
            .suggestions
            .iter()
            .filter(|s| {
                s.value.to_lowercase().contains(&input_lower)
                    || s.label
                        .as_ref()
                        .map(|l| l.to_lowercase().contains(&input_lower))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    };

    let has_suggestions = !filtered_suggestions.is_empty();

    // Build class string
    let class_string = ClassBuilder::new("el-autocomplete")
        .add_class(props.size.as_class())
        .add_if("is-disabled", props.disabled)
        .add_if("is-focus", is_focused())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let on_input = props.on_input;
    let on_select = props.on_select;
    let on_change = props.on_change;
    let on_focus = props.on_focus;
    let on_blur = props.on_blur;

    // Pre-extract suggestion data for rendering
    let suggestion_data: Vec<(usize, String, String)> = filtered_suggestions
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let display = s.label.clone().unwrap_or_else(|| s.value.clone());
            (idx, s.value.clone(), display)
        })
        .collect();

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        let key = format!("{:?}", e.key());
        match key.as_str() {
            "ArrowDown" => {
                e.prevent_default();
                if has_suggestions {
                    panel_visible.set(true);
                    let new_index = (highlighted_index() + 1) % filtered_suggestions.len();
                    highlighted_index.set(new_index);
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if has_suggestions {
                    panel_visible.set(true);
                    let len = filtered_suggestions.len();
                    let new_index = if highlighted_index() == 0 {
                        len.saturating_sub(1)
                    } else {
                        highlighted_index() - 1
                    };
                    highlighted_index.set(new_index);
                }
            }
            "Enter" => {
                e.prevent_default();
                if panel_visible() && has_suggestions {
                    if let Some(suggestion) = filtered_suggestions.get(highlighted_index()) {
                        let value = suggestion.value.clone();
                        input_value.set(value.clone());
                        fire_event(&on_select, value.clone());
                        fire_event(&on_change, value);
                        panel_visible.set(false);
                    }
                } else {
                    fire_event(&on_change, input_value());
                }
            }
            "Escape" => {
                panel_visible.set(false);
            }
            _ => {}
        }
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Input wrapper
            div {
                class: "el-input",
                onclick: move |_| {
                    if !props.disabled {
                        is_focused.set(true);
                        fire_event(&on_focus, ());
                        if props.trigger_on_focus && has_suggestions {
                            panel_visible.set(true);
                        }
                    }
                },

                // Prefix icon
                if let Some(ref icon) = props.prefix_icon {
                    span {
                        class: "el-input__prefix",
                        i { class: "{icon} el-input__icon" }
                    }
                }

                // Input element
                input {
                    class: "el-input__inner",
                    r#type: "text",
                    placeholder: "{placeholder}",
                    value: "{input_value}",
                    disabled: props.disabled,
                    oninput: move |e: FormEvent| {
                        let value = e.value();
                        input_value.set(value.clone());
                        highlighted_index.set(0);
                        panel_visible.set(true);
                        fire_event(&on_input, value);
                    },
                    onfocus: move |_| {
                        is_focused.set(true);
                        fire_event(&on_focus, ());
                        if props.trigger_on_focus && has_suggestions {
                            panel_visible.set(true);
                        }
                    },
                    onblur: move |_| {
                        is_focused.set(false);
                        panel_visible.set(false);
                        fire_event(&on_blur, ());
                    },
                    onkeydown: handle_keydown,
                }

                // Clear button
                if props.clearable && !input_value().is_empty() && !props.disabled {
                    span {
                        class: "el-input__suffix",
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            input_value.set(String::new());
                            fire_event(&on_input, String::new());
                            fire_event(&on_change, String::new());
                        },
                        i { class: "el-icon-circle-close el-input__clear" }
                    }
                }

                // Suffix icon
                if let Some(ref icon) = props.suffix_icon {
                    if !props.clearable || input_value().is_empty() {
                        span {
                            class: "el-input__suffix",
                            i { class: "{icon} el-input__icon" }
                        }
                    }
                }
            }

            // Suggestions panel
            if panel_visible() && has_suggestions {
                div {
                    class: "el-autocomplete-suggestion el-popper",
                    style: "min-width: 200px;",
                    onclick: move |e: MouseEvent| e.stop_propagation(),

                    // Loading state (placeholder for future async support)
                    div {
                        class: "el-autocomplete-suggestion__wrap",
                        ul {
                            class: "el-autocomplete-suggestion__list",
                            for item in suggestion_data.clone() {
                                {
                                    let (index, value, display_text) = item;
                                    let is_highlighted = index == highlighted_index();
                                    let class_name = if is_highlighted {
                                        "el-autocomplete-suggestion__item highlighted"
                                    } else {
                                        "el-autocomplete-suggestion__item"
                                    };
                                    let value_for_click = value.clone();
                                    let value_for_select = value.clone();
                                    let value_for_change = value.clone();
                                    rsx! {
                                        li {
                                            class: "{class_name}",
                                            onclick: move |_| {
                                                input_value.set(value_for_click.clone());
                                                fire_event(&on_select, value_for_select.clone());
                                                fire_event(&on_change, value_for_change.clone());
                                                panel_visible.set(false);
                                            },
                                            onmouseenter: move |_| {
                                                highlighted_index.set(index);
                                            },
                                            "{display_text}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete_size_as_class() {
        assert_eq!(AutocompleteSize::Default.as_class(), "");
        assert_eq!(AutocompleteSize::Large.as_class(), "el-autocomplete--large");
        assert_eq!(AutocompleteSize::Small.as_class(), "el-autocomplete--small");
    }

    #[test]
    fn test_autocomplete_suggestion() {
        let suggestion = AutocompleteSuggestion::new("apple");
        assert_eq!(suggestion.value, "apple");
        assert_eq!(suggestion.label, None);

        let suggestion_with_label = AutocompleteSuggestion::new("apple").with_label("Apple Fruit");
        assert_eq!(suggestion_with_label.value, "apple");
        assert_eq!(suggestion_with_label.label, Some("Apple Fruit".to_string()));
    }

    #[test]
    fn test_class_builder() {
        let class = ClassBuilder::new("el-autocomplete")
            .add_class("el-autocomplete--large")
            .add_if("is-disabled", true)
            .add_if("is-focus", false)
            .build();
        assert_eq!(class, "el-autocomplete el-autocomplete--large is-disabled");
    }
}
