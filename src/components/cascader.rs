use dioxus::prelude::*;

/// Cascader option
#[derive(Clone, PartialEq)]
pub struct CascaderOption {
    pub value: String,
    pub label: String,
    pub children: Vec<CascaderOption>,
    pub disabled: bool,
}

impl CascaderOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            children: vec![],
            disabled: false,
        }
    }

    pub fn child(mut self, option: CascaderOption) -> Self {
        self.children.push(option);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Cascader props
#[derive(Props, Clone, PartialEq)]
pub struct CascaderProps {
    #[props(default)]
    pub options: Vec<CascaderOption>,

    #[props(default)]
    pub model_value: Vec<String>,

    #[props(default = "Select".to_string())]
    pub placeholder: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default = false)]
    pub filterable: bool,

    #[props(default = true)]
    pub show_all_levels: bool,

    #[props(default = " / ".to_string())]
    pub separator: String,

    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Cascader node render data: (value, label, disabled, has_children, is_active)
type NodeRender = (String, String, bool, bool, bool);

/// Cascader component for multi-level selection
#[component]
pub fn Cascader(props: CascaderProps) -> Element {
    let mut class_names = vec!["el-cascader".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }

    // Build display text from model_value
    let display_text = if props.model_value.is_empty() {
        String::new()
    } else {
        let labels = resolve_path_labels(&props.options, &props.model_value);
        if props.show_all_levels {
            labels.join(&props.separator)
        } else {
            labels.last().cloned().unwrap_or_default()
        }
    };

    // Pre-compute level 1 nodes: (value, label, disabled, has_children, is_active)
    let level1_nodes: Vec<NodeRender> = props
        .options
        .iter()
        .map(|opt| {
            let is_active = props.model_value.first().map_or(false, |v| v == &opt.value);
            (opt.value.clone(), opt.label.clone(), opt.disabled, !opt.children.is_empty(), is_active)
        })
        .collect();

    // Pre-compute level 2 nodes: (value, label, disabled, has_children, is_active, l1_value)
    let level2_nodes: Vec<(String, String, bool, bool, bool, String)> = if props.model_value.len() >= 1 {
        let l1 = &props.model_value[0];
        props
            .options
            .iter()
            .find(|o| &o.value == l1)
            .map(|parent| {
                let l1_val = l1.clone();
                parent
                    .children
                    .iter()
                    .map(|opt| {
                        let is_active = props.model_value.get(1).map_or(false, |v| v == &opt.value);
                        (opt.value.clone(), opt.label.clone(), opt.disabled, !opt.children.is_empty(), is_active, l1_val.clone())
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        vec![]
    };

    // Pre-compute level 3 nodes: (value, label, disabled, has_children, is_active, l1_value, l2_value)
    let level3_nodes: Vec<(String, String, bool, bool, bool, String, String)> = if props.model_value.len() >= 2 {
        let l1 = &props.model_value[0];
        let l2 = &props.model_value[1];
        props
            .options
            .iter()
            .find(|o| &o.value == l1)
            .and_then(|p1| p1.children.iter().find(|o| &o.value == l2))
            .map(|p2| {
                let l1_val = l1.clone();
                let l2_val = l2.clone();
                p2
                    .children
                    .iter()
                    .map(|opt| {
                        let is_active = props.model_value.get(2).map_or(false, |v| v == &opt.value);
                        (opt.value.clone(), opt.label.clone(), opt.disabled, !opt.children.is_empty(), is_active, l1_val.clone(), l2_val.clone())
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        vec![]
    };

    let has_value = !props.model_value.is_empty();
    let placeholder = props.placeholder.clone();
    let show_clear = props.clearable && has_value && !props.disabled;
    let show_l1 = !level1_nodes.is_empty();
    let show_l2 = !level2_nodes.is_empty();
    let show_l3 = !level3_nodes.is_empty();
    let on_change = props.on_change;

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),

            div {
                class: "el-cascader__wrapper",

                div {
                    class: "el-input el-input--suffix el-cascader__input",

                    input {
                        class: "el-input__inner",
                        r#type: "text",
                        placeholder: "{placeholder}",
                        readonly: true,
                        disabled: props.disabled,
                        value: "{display_text}",
                    }

                    if show_clear {
                        span {
                            class: "el-input__suffix el-cascader__clear-icon",
                            onclick: move |_| {
                                if let Some(handler) = on_change.as_ref() {
                                    handler.call(vec![]);
                                }
                            },
                            i { class: "el-icon-circle-close" }
                        }
                    } else {
                        span {
                            class: "el-input__suffix el-cascader__arrow-icon",
                            i { class: "el-icon-arrow-down" }
                        }
                    }
                }
            }

            if has_value {
                div {
                    class: "el-cascader__dropdown",
                    style: "position: absolute; z-index: 2000; margin-top: 4px;",

                    div {
                        class: "el-cascader-panel",
                        style: "display: flex;",

                        if show_l1 {
                            div {
                                class: "el-cascader-menu",

                                for (value, label, disabled, has_children, is_active) in level1_nodes.into_iter() {
                                    div {
                                        class: if is_active {
                                            "el-cascader-node is-active"
                                        } else if disabled {
                                            "el-cascader-node is-disabled"
                                        } else {
                                            "el-cascader-node"
                                        },
                                        onclick: move |_| {
                                            if !disabled {
                                                if let Some(handler) = on_change.as_ref() {
                                                    handler.call(vec![value.clone()]);
                                                }
                                            }
                                        },

                                        span {
                                            class: "el-cascader-node__label",
                                            "{label}"
                                        }
                                        if has_children {
                                            i { class: "el-cascader-node__postfix el-icon-arrow-right" }
                                        }
                                    }
                                }
                            }
                        }

                        if show_l2 {
                            div {
                                class: "el-cascader-menu",

                                for (value, label, disabled, has_children, is_active, l1_val) in level2_nodes.into_iter() {
                                    div {
                                        class: if is_active {
                                            "el-cascader-node is-active"
                                        } else if disabled {
                                            "el-cascader-node is-disabled"
                                        } else {
                                            "el-cascader-node"
                                        },
                                        onclick: move |_| {
                                            if !disabled {
                                                if let Some(handler) = on_change.as_ref() {
                                                    handler.call(vec![l1_val.clone(), value.clone()]);
                                                }
                                            }
                                        },

                                        span {
                                            class: "el-cascader-node__label",
                                            "{label}"
                                        }
                                        if has_children {
                                            i { class: "el-cascader-node__postfix el-icon-arrow-right" }
                                        }
                                    }
                                }
                            }
                        }

                        if show_l3 {
                            div {
                                class: "el-cascader-menu",

                                for (value, label, disabled, _has_children, is_active, l1_val, l2_val) in level3_nodes.into_iter() {
                                    div {
                                        class: if is_active {
                                            "el-cascader-node is-active"
                                        } else if disabled {
                                            "el-cascader-node is-disabled"
                                        } else {
                                            "el-cascader-node"
                                        },
                                        onclick: move |_| {
                                            if !disabled {
                                                if let Some(handler) = on_change.as_ref() {
                                                    handler.call(vec![l1_val.clone(), l2_val.clone(), value.clone()]);
                                                }
                                            }
                                        },

                                        span {
                                            class: "el-cascader-node__label",
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

/// Resolve labels for a path of values
fn resolve_path_labels(options: &[CascaderOption], path: &[String]) -> Vec<String> {
    let mut labels = vec![];
    let mut current_options = options;

    for value in path {
        if let Some(opt) = current_options.iter().find(|o| &o.value == value) {
            labels.push(opt.label.clone());
            current_options = &opt.children;
        } else {
            break;
        }
    }

    labels
}
