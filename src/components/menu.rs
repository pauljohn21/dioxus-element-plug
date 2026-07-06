use dioxus::prelude::*;

/// Menu mode
#[derive(Clone, PartialEq)]
pub enum MenuMode {
    Vertical,
    Horizontal,
}

/// Menu props - 导航菜单
#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    /// Menu items
    pub children: Element,

    /// Menu mode
    #[props(default = MenuMode::Vertical)]
    pub mode: MenuMode,

    /// Default active index
    #[props(default)]
    pub default_active: Option<String>,

    /// Whether collapsed (vertical mode only)
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

    /// Select handler
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Menu component
#[component]
pub fn Menu(props: MenuProps) -> Element {
    let mut class_names = vec!["el-menu".to_string()];

    match props.mode {
        MenuMode::Vertical => {}
        MenuMode::Horizontal => class_names.push("el-menu--horizontal".to_string()),
    }

    if props.collapse {
        class_names.push("el-menu--collapse".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let mut style_parts = vec![props.style.unwrap_or_default()];
    if let Some(ref bg) = props.background_color {
        style_parts.push(format!("background-color: {};", bg));
    }
    let style_string = style_parts.join("");

    rsx! {
        ul {
            class: "{class_string}",
            style: "{style_string}",
            role: "menubar",
            {props.children}
        }
    }
}

/// MenuItem props
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    /// Item content
    pub children: Element,

    /// Item index
    #[props(default)]
    pub index: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<String>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// MenuItem component
#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let mut class_names = vec!["el-menu-item".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let on_click = props.on_click;
    let index = props.index.clone();
    let disabled = props.disabled;

    rsx! {
        li {
            class: "{class_string}",
            role: "menuitem",
            tabindex: "0",
            onclick: move |_| {
                if !disabled {
                    if let Some(handler) = on_click.as_ref() {
                        if let Some(ref idx) = index {
                            handler.call(idx.clone());
                        }
                    }
                }
            },

            span {
                class: "el-menu-item__content",
                {props.children}
            }
        }
    }
}

/// SubMenu props
#[derive(Props, Clone, PartialEq)]
pub struct SubMenuProps {
    /// SubMenu content
    pub children: Element,

    /// SubMenu index
    #[props(default)]
    pub index: Option<String>,

    /// SubMenu title
    #[props(default)]
    pub title: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// SubMenu component
#[component]
pub fn SubMenu(props: SubMenuProps) -> Element {
    let mut class_names = vec!["el-sub-menu".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        li {
            class: "{class_string}",

            div {
                class: "el-sub-menu__title",
                if let Some(ref t) = props.title {
                    "{t}"
                }
                " ▾"
            }

            ul {
                class: "el-menu",
                role: "menu",
                {props.children}
            }
        }
    }
}

/// MenuItemGroup props
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemGroupProps {
    /// Group items
    pub children: Element,

    /// Group title
    #[props(default)]
    pub title: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// MenuItemGroup component
#[component]
pub fn MenuItemGroup(props: MenuItemGroupProps) -> Element {
    let mut class_names = vec!["el-menu-item-group".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        li {
            class: "{class_string}",

            div {
                class: "el-menu-item-group__title",
                if let Some(ref t) = props.title {
                    "{t}"
                }
            }

            ul {
                {props.children}
            }
        }
    }
}
