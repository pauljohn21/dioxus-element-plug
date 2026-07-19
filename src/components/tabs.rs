use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Tab type
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TabType {
    #[default]
    Default,
    Card,
    BorderCard,
}

impl TabType {
    pub fn as_class(&self) -> &'static str {
        match self {
            TabType::Default => "",
            TabType::Card => "el-tabs--card",
            TabType::BorderCard => "el-tabs--border-card",
        }
    }
}

/// Tab position
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TabPosition {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TabPosition {
    pub fn as_class(&self) -> &'static str {
        match self {
            TabPosition::Top => "el-tabs--top",
            TabPosition::Right => "el-tabs--right",
            TabPosition::Bottom => "el-tabs--bottom",
            TabPosition::Left => "el-tabs--left",
        }
    }
}

/// Tabs props
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// TabPane children
    #[props(default)]
    pub children: Element,

    /// Currently active tab name
    #[props(default)]
    pub model_value: Option<String>,

    /// Tab type (default, card, border-card)
    #[props(default = TabType::Default)]
    pub tab_type: TabType,

    /// Tab position (top, right, bottom, left)
    #[props(default = TabPosition::Top)]
    pub tab_position: TabPosition,

    /// Whether tabs are closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether tabs are addable
    #[props(default = false)]
    pub addable: bool,

    /// Whether tabs are editable (both closable and addable)
    #[props(default = false)]
    pub editable: bool,

    /// Tab click callback
    #[props(default)]
    pub on_tab_click: Option<EventHandler<String>>,

    /// Tab change callback
    #[props(default)]
    pub on_tab_change: Option<EventHandler<String>>,

    /// Tab remove callback
    #[props(default)]
    pub on_tab_remove: Option<EventHandler<String>>,

    /// Tab add callback
    #[props(default)]
    pub on_tab_add: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tabs component for tabbed navigation
///
/// Provides tabbed navigation with support for different styles,
/// positions, and interactive features like closing and adding tabs.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::tabs::{Tabs, TabPane, TabType};
///
/// rsx! {
///     Tabs {
///         model_value: Some("tab1".to_string()),
///         tab_type: TabType::Card,
///         TabPane {
///             label: "Tab 1".to_string(),
///             name: Some("tab1".to_string()),
///             "Content of Tab 1"
///         }
///         TabPane {
///             label: "Tab 2".to_string(),
///             name: Some("tab2".to_string()),
///             "Content of Tab 2"
///         }
///     }
/// }
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    // Build class string using ClassBuilder
    let class_string = ClassBuilder::new("el-tabs")
        .add_class(props.tab_type.as_class())
        .add_class(props.tab_position.as_class())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    // Determine if we need to show add button
    let show_add = props.addable || props.editable;

    let on_tab_add = props.on_tab_add;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Tab header will be rendered by TabPane children
            // We use a context-like pattern where TabPane renders both the tab and content

            // Tab content area
            div {
                class: "el-tabs__content",
                {props.children}
            }

            // Add button (if addable)
            if show_add {
                div {
                    class: "el-tabs__add-tab",
                    button {
                        class: "el-button el-button--text",
                        onclick: move |_| {
                            fire_event(&on_tab_add, ());
                        },
                        i { class: "el-icon-plus" }
                        "Add"
                    }
                }
            }
        }
    }
}

/// TabPane props
#[derive(Props, Clone, PartialEq)]
pub struct TabPaneProps {
    /// Tab content
    #[props(default)]
    pub children: Element,

    /// Tab label text (required)
    pub label: String,

    /// Tab name/identifier (defaults to label if not provided)
    #[props(default)]
    pub name: Option<String>,

    /// Whether the tab is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the tab is closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether the tab is lazy loaded
    #[props(default = false)]
    pub lazy: bool,

    /// Whether this tab is active (set by parent Tabs)
    #[props(default = false)]
    pub is_active: bool,

    /// Tab click callback
    #[props(default)]
    pub on_tab_click: Option<EventHandler<String>>,

    /// Tab remove callback
    #[props(default)]
    pub on_tab_remove: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// TabPane component for individual tab panels
///
/// Represents a single tab with its content. The label is displayed
/// in the tab header, and children are shown when the tab is active.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::tabs::TabPane;
///
/// rsx! {
///     TabPane {
///         label: "Settings".to_string(),
///         name: Some("settings".to_string()),
///         disabled: false,
///         closable: true,
///         "Settings content here"
///     }
/// }
/// ```
#[component]
pub fn TabPane(props: TabPaneProps) -> Element {
    // Get tab name (use label if name not provided)
    let tab_name = props.name.clone().unwrap_or_else(|| props.label.clone());
    let tab_name_for_click = tab_name.clone();
    let tab_name_for_remove = tab_name.clone();

    // Build class string
    let class_string = ClassBuilder::new("el-tab-pane")
        .add_if("is-active", props.is_active)
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let on_tab_click = props.on_tab_click;
    let on_tab_remove = props.on_tab_remove;

    rsx! {
        // Tab header item (simplified - in real implementation,
        // this would be collected by Tabs parent)
        if props.is_active {
            div {
                class: "{class_string}",
                style: "{style_string}",
                role: "tabpanel",

                // Tab header representation
                div {
                    class: "el-tabs__item is-active",
                    onclick: move |_| {
                        if !props.disabled {
                            fire_event(&on_tab_click, tab_name_for_click.clone());
                        }
                    },
                    "{props.label}"
                    if props.closable {
                        span {
                            class: "el-icon-close",
                            onclick: move |e: MouseEvent| {
                                e.stop_propagation();
                                fire_event(&on_tab_remove, tab_name_for_remove.clone());
                            },
                        }
                    }
                }

                // Tab content
                div {
                    class: "el-tab-pane__content",
                    {props.children}
                }
            }
        } else {
            // Inactive tab - only show header in real implementation
            // For now, render nothing when inactive
            // (In full implementation, Tabs would collect all headers)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_type_as_class() {
        assert_eq!(TabType::Default.as_class(), "");
        assert_eq!(TabType::Card.as_class(), "el-tabs--card");
        assert_eq!(TabType::BorderCard.as_class(), "el-tabs--border-card");
    }

    #[test]
    fn test_tab_position_as_class() {
        assert_eq!(TabPosition::Top.as_class(), "el-tabs--top");
        assert_eq!(TabPosition::Right.as_class(), "el-tabs--right");
        assert_eq!(TabPosition::Bottom.as_class(), "el-tabs--bottom");
        assert_eq!(TabPosition::Left.as_class(), "el-tabs--left");
    }

    #[test]
    fn test_tab_type_default() {
        assert_eq!(TabType::default(), TabType::Default);
    }

    #[test]
    fn test_tab_position_default() {
        assert_eq!(TabPosition::default(), TabPosition::Top);
    }

    #[test]
    fn test_class_builder_tabs() {
        let class = ClassBuilder::new("el-tabs")
            .add_class("el-tabs--card")
            .add_class("el-tabs--top")
            .build();
        assert_eq!(class, "el-tabs el-tabs--card el-tabs--top");
    }

    #[test]
    fn test_class_builder_tab_pane() {
        let class = ClassBuilder::new("el-tab-pane")
            .add_if("is-active", true)
            .add_if("is-disabled", false)
            .build();
        assert_eq!(class, "el-tab-pane is-active");
    }
}
