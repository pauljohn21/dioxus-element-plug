use dioxus::prelude::*;

/// Menu mode
#[derive(Clone, PartialEq)]
pub enum MenuMode {
    Horizontal,
    Vertical,
}

impl MenuMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuMode::Horizontal => "horizontal",
            MenuMode::Vertical => "vertical",
        }
    }
}

/// Menu props
#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    #[props(default)]
    pub children: Element,

    #[props(default = MenuMode::Vertical)]
    pub mode: MenuMode,

    /// Index of the active menu item
    #[props(default)]
    pub default_active: Option<String>,

    /// Whether the menu is collapsed (vertical mode only)
    #[props(default = false)]
    pub collapse: bool,

    /// Background color
    #[props(default)]
    pub background_color: Option<String>,

    /// Text color
    #[props(default)]
    pub text_color: Option<String>,

    /// Active text color
    #[props(default)]
    pub active_text_color: Option<String>,

    /// Whether to use unique opened mode
    #[props(default = false)]
    pub unique_opened: bool,

    /// Select event handler
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Menu component for navigation
#[component]
pub fn Menu(props: MenuProps) -> Element {
    let mut class_names = vec!["el-menu".to_string()];
    class_names.push(format!("el-menu--{}", props.mode.as_str()));
    if props.collapse { class_names.push("el-menu--collapse".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let mut style = props.style.clone().unwrap_or_default();
    if let Some(ref bg) = props.background_color { style = format!("background-color: {}; {}", bg, style); }

    rsx! {
        ul {
            class: "{class_names.join(\" \")}",
            style: "{style}",
            role: "menubar",
            {props.children}
        }
    }
}

/// MenuItem props
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Unique index for this menu item
    pub index: String,

    /// Route to navigate to
    #[props(default)]
    pub route: Option<String>,

    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Select event handler
    #[props(default)]
    pub on_click: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,
}

/// MenuItem component for individual menu items
#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let mut class_names = vec!["el-menu-item".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let index_clone = props.index.clone();

    rsx! {
        li {
            class: "{class_names.join(\" \")}",
            role: "menuitem",
            onclick: move |_| {
                if !props.disabled {
                    if let Some(handler) = props.on_click {
                        handler.call(index_clone.clone());
                    }
                }
            },
            {props.children}
        }
    }
}

/// MenuItemGroup props
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemGroupProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub title: Option<String>,

    #[props(default)]
    pub class: Option<String>,
}

/// MenuItemGroup component for grouping menu items
#[component]
pub fn MenuItemGroup(props: MenuItemGroupProps) -> Element {
    let mut class_names = vec!["el-menu-item-group".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        li {
            class: "{class_names.join(\" \")}",
            role: "menuitemgroup",
            if let Some(ref title) = props.title {
                div { class: "el-menu-item-group__title", "{title}" }
            }
            ul {
                class: "el-menu-item-group__content",
                {props.children}
            }
        }
    }
}

/// SubMenu props
#[derive(Props, Clone, PartialEq)]
pub struct SubMenuProps {
    #[props(default)]
    pub children: Element,

    pub index: String,

    #[props(default)]
    pub title: Option<String>,

    #[props(default)]
    pub icon: Option<String>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default)]
    pub class: Option<String>,
}

/// SubMenu component for nested menu items
#[component]
pub fn SubMenu(props: SubMenuProps) -> Element {
    let mut class_names = vec!["el-sub-menu".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        li {
            class: "{class_names.join(\" \")}",
            role: "menuitem",
            div {
                class: "el-sub-menu__title",
                if let Some(ref icon) = props.icon {
                    i { class: "{icon}" }
                }
                if let Some(ref title) = props.title {
                    span { "{title}" }
                }
                i { class: "el-sub-menu__icon-arrow el-icon-arrow-down" }
            }
            ul {
                class: "el-menu el-menu--inline",
                role: "menu",
                {props.children}
            }
        }
    }
}
