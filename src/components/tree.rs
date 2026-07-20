use dioxus::prelude::*;
use std::collections::HashSet;

use crate::components::common::{style_str, ClassBuilder};

/// Tree node data
#[derive(Clone, PartialEq)]
pub struct TreeNodeData {
    pub label: String,
    pub children: Vec<TreeNodeData>,
    pub disabled: bool,
}

impl TreeNodeData {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            children: vec![],
            disabled: false,
        }
    }

    pub fn child(mut self, node: TreeNodeData) -> Self {
        self.children.push(node);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Tree props
#[derive(Props, Clone, PartialEq)]
pub struct TreeProps {
    /// Tree data
    #[props(default)]
    pub data: Vec<TreeNodeData>,

    /// Whether to show checkboxes
    #[props(default = false)]
    pub show_checkbox: bool,

    /// Whether to expand all nodes by default
    #[props(default = false)]
    pub default_expand_all: bool,

    /// Currently expanded node labels
    #[props(default)]
    pub expanded_keys: Vec<String>,

    /// Currently checked node labels
    #[props(default)]
    pub checked_keys: Vec<String>,

    /// Currently highlighted node label
    #[props(default)]
    pub current_key: Option<String>,

    /// Whether to highlight current node
    #[props(default = false)]
    pub highlight_current: bool,

    /// Whether to expand on node click
    #[props(default = true)]
    pub expand_on_click_node: bool,

    /// Node click handler
    #[props(default)]
    pub on_node_click: Option<EventHandler<String>>,

    /// Node expand/collapse toggle handler
    #[props(default)]
    pub on_node_expand: Option<EventHandler<(String, bool)>>,

    /// Node check handler (for checkboxes)
    #[props(default)]
    pub on_node_check: Option<EventHandler<(String, bool)>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Tree component for hierarchical data
///
/// This component renders a tree structure with optional expand/collapse,
/// checkboxes, and node highlighting.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::tree::{Tree, TreeNodeData};
///
/// let data = vec![
///     TreeNodeData::new("Level 1")
///         .child(TreeNodeData::new("Level 2-1"))
///         .child(TreeNodeData::new("Level 2-2")),
/// ];
///
/// rsx! {
///     Tree {
///         data: data,
///         default_expand_all: true,
///         highlight_current: true,
///     }
/// }
/// ```
#[component]
pub fn Tree(props: TreeProps) -> Element {
    let class_string = ClassBuilder::new("el-tree")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    // Build expanded set from props
    let expanded_set: HashSet<String> = if props.default_expand_all {
        collect_all_labels(&props.data).into_iter().collect()
    } else {
        props.expanded_keys.iter().cloned().collect()
    };

    let checked_set: HashSet<String> = props.checked_keys.iter().cloned().collect();
    let current_key = props.current_key.clone();

    // Pre-compute top-level node render data
    let node_data: Vec<TreeNodeRenderData> = props
        .data
        .iter()
        .map(|node| {
            build_node_render_data(
                node,
                0,
                &expanded_set,
                &checked_set,
                &current_key,
                props.highlight_current,
                props.show_checkbox,
            )
        })
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "tree",

            for render_node in node_data.into_iter() {
                TreeChild {
                    node: render_node,
                    show_checkbox: props.show_checkbox,
                    highlight_current: props.highlight_current,
                    expand_on_click: props.expand_on_click_node,
                    on_click: props.on_node_click,
                    on_expand: props.on_node_expand,
                    on_check: props.on_node_check,
                }
            }
        }
    }
}

/// Pre-computed render data for a tree node
#[derive(Clone, PartialEq)]
struct TreeNodeRenderData {
    label: String,
    disabled: bool,
    level: u32,
    is_expanded: bool,
    is_checked: bool,
    is_current: bool,
    has_children: bool,
    children: Vec<TreeNodeRenderData>,
}

fn build_node_render_data(
    node: &TreeNodeData,
    level: u32,
    expanded: &HashSet<String>,
    checked: &HashSet<String>,
    current: &Option<String>,
    highlight: bool,
    // Reserved for future per-node checkbox display toggle
    _show_checkbox: bool,
) -> TreeNodeRenderData {
    let is_expanded = expanded.contains(&node.label);
    let is_checked = checked.contains(&node.label);
    let is_current = highlight && (current.as_ref() == Some(&node.label));
    let has_children = !node.children.is_empty();

    let children = if has_children {
        node.children
            .iter()
            .map(|child| {
                build_node_render_data(
                    child,
                    level + 1,
                    expanded,
                    checked,
                    current,
                    highlight,
                    _show_checkbox,
                )
            })
            .collect()
    } else {
        vec![]
    };

    TreeNodeRenderData {
        label: node.label.clone(),
        disabled: node.disabled,
        level,
        is_expanded,
        is_checked,
        is_current,
        has_children,
        children,
    }
}

fn collect_all_labels(data: &[TreeNodeData]) -> Vec<String> {
    let mut result = vec![];
    for node in data {
        result.push(node.label.clone());
        if !node.children.is_empty() {
            result.extend(collect_all_labels(&node.children));
        }
    }
    result
}

#[derive(Props, Clone, PartialEq)]
struct TreeChildProps {
    node: TreeNodeRenderData,
    show_checkbox: bool,
    highlight_current: bool,
    expand_on_click: bool,
    on_click: Option<EventHandler<String>>,
    on_expand: Option<EventHandler<(String, bool)>>,
    on_check: Option<EventHandler<(String, bool)>>,
}

#[component]
fn TreeChild(props: TreeChildProps) -> Element {
    let node_label = props.node.label.clone();
    let padding = props.node.level * 18;
    let has_children = props.node.has_children;
    let is_expanded = props.node.is_expanded;
    let is_current = props.node.is_current;
    let is_checked = props.node.is_checked;
    let disabled = props.node.disabled;
    let show_checkbox = props.show_checkbox;
    let expand_on_click = props.expand_on_click;
    let on_click = props.on_click;
    let on_expand = props.on_expand;
    let on_check = props.on_check;

    // Clone labels for each closure that needs it
    let label_for_click = node_label.clone();
    let label_for_expand = node_label.clone();
    let label_for_expand_icon = node_label.clone();
    let label_for_check = node_label.clone();

    // Pre-compute child render data
    let child_nodes: Vec<TreeNodeRenderData> = if has_children && is_expanded {
        props.node.children.clone()
    } else {
        vec![]
    };

    // Build content class
    let content_class = ClassBuilder::new("el-tree-node__content")
        .add_if("is-current", is_current)
        .add_if("is-disabled", disabled)
        .build();

    // Build expand icon class
    let expand_icon_class = if is_expanded {
        "el-tree-node__expand-icon el-icon-caret-right expanded"
    } else {
        "el-tree-node__expand-icon el-icon-caret-right"
    };

    // Build checkbox class
    let checkbox_class = ClassBuilder::new("el-checkbox")
        .add_if("is-checked", is_checked)
        .add_class("el-tree-node__checkbox")
        .build();

    rsx! {
        div {
            class: "el-tree-node",
            role: "treeitem",
            "aria-expanded": "{is_expanded}",

            div {
                class: "{content_class}",
                style: "padding-left: {padding}px;",

                onclick: move |_| {
                    if !disabled {
                        if let Some(handler) = on_click.as_ref() {
                            handler.call(label_for_click.clone());
                        }
                        if expand_on_click && has_children {
                            if let Some(handler) = on_expand.as_ref() {
                                handler.call((label_for_expand.clone(), !is_expanded));
                            }
                        }
                    }
                },

                if has_children {
                    span {
                        class: "{expand_icon_class}",
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            if !disabled {
                                if let Some(handler) = on_expand.as_ref() {
                                    handler.call((label_for_expand_icon.clone(), !is_expanded));
                                }
                            }
                        },
                    }
                } else {
                    span {
                        class: "el-tree-node__expand-icon is-leaf",
                    }
                }

                if show_checkbox {
                    span {
                        class: "{checkbox_class}",
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            if !disabled {
                                if let Some(handler) = on_check.as_ref() {
                                    handler.call((label_for_check.clone(), !is_checked));
                                }
                            }
                        },
                        span { class: "el-checkbox__inner" }
                    }
                }

                span {
                    class: "el-tree-node__label",
                    "{props.node.label}"
                }
            }

            if has_children && is_expanded {
                div {
                    class: "el-tree-node__children",
                    role: "group",

                    for child in child_nodes.into_iter() {
                        TreeChild {
                            node: child,
                            show_checkbox: show_checkbox,
                            highlight_current: props.highlight_current,
                            expand_on_click: expand_on_click,
                            on_click: on_click,
                            on_expand: on_expand,
                            on_check: on_check,
                        }
                    }
                }
            }
        }
    }
}
