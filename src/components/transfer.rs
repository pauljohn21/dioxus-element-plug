use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str, fire_event};

#[cfg(feature = "icons")]
use element_icons::element::{ArrowLeft, ArrowRight};

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

    // Track selected keys in left and right panels
    let mut left_selected: Signal<Vec<String>> = use_signal(Vec::new);
    let mut right_selected: Signal<Vec<String>> = use_signal(Vec::new);

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

    // Clone for closures
    let _left_items_clone = left_items.clone();
    let right_items_clone = right_items.clone();
    let current_right_keys: Vec<String> = props.model_value.clone();

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
                            {
                                let key_clone = key.clone();
                                let is_selected = left_selected().contains(&key);
                                rsx! {
                                    label {
                                        class: "el-transfer-panel__item",
                                        input {
                                            r#type: "checkbox",
                                            disabled: disabled,
                                            checked: is_selected,
                                            onchange: move |evt: Event<FormData>| {
                                                let checked = evt.data().checked();
                                                let mut selected = left_selected();
                                                if checked {
                                                    if !selected.contains(&key_clone) {
                                                        selected.push(key_clone.clone());
                                                    }
                                                } else {
                                                    selected.retain(|k| k != &key_clone);
                                                }
                                                left_selected.set(selected);
                                            },
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

            // Transfer buttons
            {
                let left_selected_keys = left_selected();
                let right_selected_keys = right_selected();
                let has_left_selection = !left_selected_keys.is_empty();
                let has_right_selection = !right_selected_keys.is_empty();

                let move_right = {
                    let left_selected_keys = left_selected_keys.clone();
                    let current_right_keys = current_right_keys.clone();
                    let on_change = on_change.clone();
                    move |_| {
                        // Move selected left items to right
                        let mut new_right = current_right_keys.clone();
                        for key in &left_selected_keys {
                            if !new_right.contains(key) {
                                new_right.push(key.clone());
                            }
                        }
                        fire_event(&on_change, new_right);
                        // Clear left selection after move
                        left_selected.set(vec![]);
                    }
                };

                let move_left = {
                    let right_selected_keys = right_selected_keys.clone();
                    let current_right_keys = current_right_keys.clone();
                    move |_| {
                        // Move selected right items back to left (remove from right)
                        let new_right: Vec<String> = current_right_keys
                            .iter()
                            .filter(|k| !right_selected_keys.contains(k))
                            .cloned()
                            .collect();
                        fire_event(&on_change, new_right);
                        // Clear right selection after move
                        right_selected.set(vec![]);
                    }
                };

                render_buttons(left_count, right_count, has_left_selection, has_right_selection, move_right, move_left)
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
                        for (key, label, disabled) in right_items_clone.into_iter() {
                            {
                                let key_clone = key.clone();
                                let is_selected = right_selected().contains(&key);
                                rsx! {
                                    label {
                                        class: "el-transfer-panel__item",
                                        input {
                                            r#type: "checkbox",
                                            disabled: disabled,
                                            checked: is_selected,
                                            onchange: move |evt: Event<FormData>| {
                                                let checked = evt.data().checked();
                                                let mut selected = right_selected();
                                                if checked {
                                                    if !selected.contains(&key_clone) {
                                                        selected.push(key_clone.clone());
                                                    }
                                                } else {
                                                    selected.retain(|k| k != &key_clone);
                                                }
                                                right_selected.set(selected);
                                            },
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
    }
}

/// Render transfer buttons with or without icons based on feature flags.
#[cfg(feature = "icons")]
fn render_buttons<F1, F2>(
    left_count: usize,
    right_count: usize,
    has_left_selection: bool,
    has_right_selection: bool,
    move_right: F1,
    move_left: F2,
) -> Element
where
    F1: FnMut(MouseEvent) + 'static,
    F2: FnMut(MouseEvent) + 'static,
{
    rsx! {
        div {
            class: "el-transfer__buttons",
            button {
                class: "el-button el-transfer__button el-button--primary",
                r#type: "button",
                onclick: move_right,
                disabled: left_count == 0 || !has_left_selection,
                ArrowRight {}
            }
            button {
                class: "el-button el-transfer__button el-button--primary",
                r#type: "button",
                onclick: move_left,
                disabled: right_count == 0 || !has_right_selection,
                ArrowLeft {}
            }
        }
    }
}

/// Render transfer buttons with text fallback when icons feature is disabled.
#[cfg(not(feature = "icons"))]
fn render_buttons<F1, F2>(
    left_count: usize,
    right_count: usize,
    has_left_selection: bool,
    has_right_selection: bool,
    move_right: F1,
    move_left: F2,
) -> Element
where
    F1: FnMut(MouseEvent) + 'static,
    F2: FnMut(MouseEvent) + 'static,
{
    rsx! {
        div {
            class: "el-transfer__buttons",
            button {
                class: "el-button el-transfer__button el-button--primary",
                r#type: "button",
                onclick: move_right,
                disabled: left_count == 0 || !has_left_selection,
                ">"
            }
            button {
                class: "el-button el-transfer__button el-button--primary",
                r#type: "button",
                onclick: move_left,
                disabled: right_count == 0 || !has_right_selection,
                "<"
            }
        }
    }
}
