use dioxus::prelude::*;

/// Dropdown trigger type
#[derive(Clone, PartialEq)]
pub enum DropdownTrigger {
    Hover,
    Click,
}

/// Dropdown props - 下拉菜单
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// Trigger element
    pub children: Element,

    /// Trigger type
    #[props(default = DropdownTrigger::Hover)]
    pub trigger: DropdownTrigger,

    /// Whether visible (controlled)
    #[props(default = false)]
    pub visible: bool,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Size
    #[props(default = "default".to_string())]
    pub size: String,

    /// Command handler
    #[props(default)]
    pub on_command: Option<EventHandler<String>>,

    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Visible change handler
    #[props(default)]
    pub on_visible_change: Option<EventHandler<bool>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Dropdown component
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut class_names = vec!["el-dropdown".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",

            {props.children}
        }
    }
}

/// DropdownMenu props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// Menu items
    pub children: Element,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// DropdownMenu component
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut class_names = vec!["el-dropdown-menu".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        ul {
            class: "{class_string}",
            role: "menu",
            {props.children}
        }
    }
}

/// DropdownItem props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps {
    /// Item content
    pub children: Element,

    /// Command value
    #[props(default)]
    pub command: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether divided
    #[props(default = false)]
    pub divided: bool,

    /// Icon class
    #[props(default)]
    pub icon: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// DropdownItem component
#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let mut class_names = vec!["el-dropdown-menu__item".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if props.divided {
        class_names.push("is-divided".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        li {
            class: "{class_string}",
            role: "menuitem",
            tabindex: if props.disabled { "-1" } else { "0" },

            if let Some(ref icon) = props.icon {
                i { class: "{icon}" }
            }

            {props.children}
        }
    }
}
