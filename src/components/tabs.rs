use dioxus::prelude::*;

/// Tab type
#[derive(Clone, PartialEq)]
pub enum TabType {
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
#[derive(Clone, PartialEq)]
pub enum TabPosition {
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
    #[props(default)]
    pub children: Element,

    /// Currently active tab name
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = TabType::Default)]
    pub tab_type: TabType,

    #[props(default = TabPosition::Top)]
    pub tab_position: TabPosition,

    #[props(default = false)]
    pub closable: bool,

    #[props(default = false)]
    pub addable: bool,

    #[props(default = false)]
    pub editable: bool,

    #[props(default)]
    pub on_tab_click: Option<EventHandler<String>>,

    #[props(default)]
    pub on_tab_change: Option<EventHandler<String>>,

    #[props(default)]
    pub on_tab_remove: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Tabs component for tabbed navigation
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut class_names = vec!["el-tabs".to_string()];
    let type_class = props.tab_type.as_class();
    if !type_class.is_empty() { class_names.push(type_class.to_string()); }
    class_names.push(props.tab_position.as_class().to_string());
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
}

/// TabPane props
#[derive(Props, Clone, PartialEq)]
pub struct TabPaneProps {
    #[props(default)]
    pub children: Element,

    /// Tab label text
    pub label: String,

    /// Tab name/identifier
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

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// TabPane component for individual tab panels
#[component]
pub fn TabPane(props: TabPaneProps) -> Element {
    let mut class_names = vec!["el-tab-pane".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            role: "tabpanel",
            {props.children}
        }
    }
}
