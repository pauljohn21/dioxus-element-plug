use dioxus::prelude::*;

/// Dropdown trigger type
#[derive(Clone, PartialEq)]
pub enum DropdownTrigger {
    Hover,
    Click,
    ContextMenu,
}

/// Dropdown props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    #[props(default)]
    pub children: Element,

    #[props(default = DropdownTrigger::Hover)]
    pub trigger: DropdownTrigger,

    #[props(default = "bottom".to_string())]
    pub placement: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = 0)]
    pub max_height: u32,

    #[props(default)]
    pub on_command: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Dropdown component for dropdown menus
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut class_names = vec!["el-dropdown".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
}

/// DropdownItem props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps {
    #[props(default)]
    pub children: Option<Element>,

    #[props(default)]
    pub command: Option<String>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub divided: bool,

    #[props(default = false)]
    pub selected: bool,

    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    #[props(default)]
    pub class: Option<String>,
}

/// DropdownItem component for individual dropdown menu items
#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let mut class_names = vec!["el-dropdown-menu__item".to_string()];
    if props.disabled { class_names.push("is-disabled".to_string()); }
    if props.divided { class_names.push("is-divided".to_string()); }
    if props.selected { class_names.push("is-selected".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let cmd_clone = props.command.clone();

    rsx! {
        li {
            class: "{class_names.join(\" \")}",
            role: "menuitem",
            onclick: move |e| {
                if !props.disabled {
                    if let Some(handler) = props.on_click {
                        handler.call(e);
                    }
                }
            },
            {props.children}
            {cmd_clone.map(|c| rsx! { span { class: "el-dropdown-menu__command", style: "display:none;", "{c}" } })}
        }
    }
}

/// DropdownMenu props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// DropdownMenu component - container for dropdown items
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut class_names = vec!["el-dropdown-menu".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        ul {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            role: "menu",
            {props.children}
        }
    }
}
