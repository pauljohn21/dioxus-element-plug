use dioxus::prelude::*;

/// Tabs type
#[derive(Clone, PartialEq)]
pub enum TabsType {
    Default,
    Card,
    BorderCard,
}

impl TabsType {
    pub fn as_class(&self) -> &'static str {
        match self {
            TabsType::Default => "",
            TabsType::Card => "el-tabs--card",
            TabsType::BorderCard => "el-tabs--border-card",
        }
    }
}

/// Tabs props - 标签页
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Tabs content (TabPane components)
    pub children: Element,

    /// Active tab name
    #[props(default)]
    pub model_value: Option<String>,

    /// Tabs type
    #[props(default = TabsType::Default)]
    pub r#type: TabsType,

    /// Tab position
    #[props(default = "top".to_string())]
    pub tab_position: String,

    /// Whether closable
    #[props(default = false)]
    pub closable: bool,

    /// Whether to add tab
    #[props(default = false)]
    pub addable: bool,

    /// Tab click handler
    #[props(default)]
    pub on_tab_click: Option<EventHandler<String>>,

    /// Tab change handler
    #[props(default)]
    pub on_tab_change: Option<EventHandler<String>>,

    /// Tab remove handler
    #[props(default)]
    pub on_tab_remove: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tabs component
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut class_names = vec!["el-tabs".to_string()];
    class_names.push(props.r#type.as_class().to_string());
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
