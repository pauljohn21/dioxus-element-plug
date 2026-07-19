use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str};

/// Transfer data item
#[derive(Clone, PartialEq)]
pub struct TransferItem {
    pub key: String,
    pub label: String,
    pub disabled: bool,
}

impl TransferItem {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Transfer props
#[derive(Props, Clone, PartialEq)]
pub struct TransferProps {
    /// All available data items
    #[props(default)]
    pub data: Vec<TransferItem>,

    /// Keys currently in the right panel
    #[props(default)]
    pub model_value: Vec<String>,

    /// Whether the transfer is filterable
    #[props(default = false)]
    pub filterable: bool,

    /// Panel titles [left_title, right_title]
    #[props(default)]
    pub titles: Vec<String>,

    /// Filter placeholder text
    #[props(default = "Enter keyword".to_string())]
    pub filter_placeholder: String,

    /// Empty state text
    #[props(default = "No Data".to_string())]
    pub empty_text: String,

    /// Change handler - receives the right panel keys
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    /// Left panel selection change handler
    #[props(default)]
    pub on_left_check: Option<EventHandler<Vec<String>>>,

    /// Right panel selection change handler
    #[props(default)]
    pub on_right_check: Option<EventHandler<Vec<String>>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Transfer component for moving items between two panels
///
/// This component provides a dual-panel transfer list where users can move
/// items between left (available) and right (selected) panels.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::transfer::{Transfer, TransferItem};
///
/// let data = vec![
///     TransferItem::new("1", "Option 1"),
///     TransferItem::new("2", "Option 2"),
///     TransferItem::new("3", "Option 3"),
/// ];
///
/// rsx! {
///     Transfer {
///         data: data,
///         model_value: vec!["1".to_string()],
///         titles: vec!["Available".to_string(), "Selected".to_string()],
///     }
/// }
/// ```
#[component]
pub fn Transfer(props: TransferProps) -> Element {
    let class_string = ClassBuilder::new("el-transfer")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    // Split data into left and right panels
    let right_keys: std::collections::HashSet<&String> = props.model_value.iter().collect();

    let left_items: Vec<(String, String, bool)> = props
        .data
        .iter()
        .filter(|item| !right_keys.contains(&item.key))
        .map(|item| (item.key.clone(), item.label.clone(), item.disabled))
        .collect();

    let right_items: Vec<(String, String, bool)> = props
        .data
        .iter()
        .filter(|item| right_keys.contains(&item.key))
        .map(|item| (item.key.clone(), item.label.clone(), item.disabled))
        .collect();

    let left_title = props.titles.first().cloned().unwrap_or_else(|| "List 1".to_string());
    let right_title = props.titles.get(1).cloned().unwrap_or_else(|| "List 2".to_string());

    let left_count = left_items.len();
    let right_count = right_items.len();
    let left_empty = left_items.is_empty();
    let right_empty = right_items.is_empty();
    let empty_text = props.empty_text.clone();
    let filterable = props.filterable;
    let filter_placeholder = props.filter_placeholder.clone();

    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Left panel
            div {
                class: "el-transfer-panel",

                p {
                    class: "el-transfer-panel__header",
                    span {
                        class: "el-checkbox__label",
                        "{left_title}"
                    }
                    span {
                        class: "el-transfer-panel__header-count",
                        "{left_count} items"
                    }
                }

                if filterable {
                    div {
                        class: "el-transfer-panel__filter",
                        input {
                            class: "el-input__inner",
                            r#type: "text",
                            placeholder: "{filter_placeholder}",
                        }
                    }
                }

                div {
                    class: "el-transfer-panel__body",

                    if left_empty {
                        p {
                            class: "el-transfer-panel__empty",
                            "{empty_text}"
                        }
                    } else {
                        for (key, label, disabled) in left_items.into_iter() {
                            label {
                                class: "el-transfer-panel__item",
                                input {
                                    r#type: "checkbox",
                                    disabled: disabled,
                                    value: "{key}",
                                }
                                span {
                                    class: "el-checkbox__label",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Transfer buttons
            div {
                class: "el-transfer__buttons",

                button {
                    class: "el-button el-transfer__button el-button--primary",
                    r#type: "button",
                    onclick: move |_| {
                        // Move selected items from left to right
                        // For simplicity, move all non-disabled items
                        let new_right: Vec<String> = props.data.iter()
                            .filter(|item| !item.disabled)
                            .map(|item| item.key.clone())
                            .collect();
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(new_right);
                        }
                    },
                    disabled: left_count == 0,
                    i { class: "el-icon-arrow-right" }
                }

                button {
                    class: "el-button el-transfer__button el-button--primary",
                    r#type: "button",
                    onclick: move |_| {
                        // Move all items back to left (clear right panel)
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(vec![]);
                        }
                    },
                    disabled: right_count == 0,
                    i { class: "el-icon-arrow-left" }
                }
            }

            // Right panel
            div {
                class: "el-transfer-panel",

                p {
                    class: "el-transfer-panel__header",
                    span {
                        class: "el-checkbox__label",
                        "{right_title}"
                    }
                    span {
                        class: "el-transfer-panel__header-count",
                        "{right_count} items"
                    }
                }

                if filterable {
                    div {
                        class: "el-transfer-panel__filter",
                        input {
                            class: "el-input__inner",
                            r#type: "text",
                            placeholder: "{filter_placeholder}",
                        }
                    }
                }

                div {
                    class: "el-transfer-panel__body",

                    if right_empty {
                        p {
                            class: "el-transfer-panel__empty",
                            "{empty_text}"
                        }
                    } else {
                        for (key, label, disabled) in right_items.into_iter() {
                            label {
                                class: "el-transfer-panel__item",
                                input {
                                    r#type: "checkbox",
                                    disabled: disabled,
                                    value: "{key}",
                                    checked: true,
                                }
                                span {
                                    class: "el-checkbox__label",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
