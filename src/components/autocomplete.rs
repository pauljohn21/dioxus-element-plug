use dioxus::prelude::*;

/// Autocomplete props
#[derive(Props, Clone, PartialEq)]
pub struct AutocompleteProps {
    #[props(default = "placeholder".to_string())]
    pub placeholder: String,

    #[props(default)]
    pub value: Option<String>,

    #[props(default)]
    pub suggestions: Vec<String>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default)]
    pub on_select: Option<EventHandler<String>>,

    #[props(default)]
    pub on_input: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Autocomplete component for input with suggestions
#[component]
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    let mut class_names = vec!["el-autocomplete".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let suggestions_owned = props.suggestions.clone();
    let has_suggestions = !suggestions_owned.is_empty();

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            div {
                class: "el-input",
                input {
                    class: "el-input__inner",
                    r#type: "text",
                    placeholder: "{props.placeholder}",
                    value: props.value.clone().unwrap_or_default(),
                    disabled: props.disabled,
                    oninput: move |e| {
                        if let Some(handler) = props.on_input {
                            handler.call(e.value());
                        }
                    },
                }
            }
            if has_suggestions {
                div {
                    class: "el-autocomplete-suggestion el-popper",
                    style: "min-width: 200px;",
                    ul {
                        class: "el-autocomplete-suggestion__list",
                        for suggestion in suggestions_owned.into_iter() {
                            li {
                                class: "el-autocomplete-suggestion__item",
                                onclick: move |_| {
                                    if let Some(handler) = props.on_select {
                                        handler.call(suggestion.clone());
                                    }
                                },
                                "{suggestion}"
                            }
                        }
                    }
                }
            }
        }
    }
}
