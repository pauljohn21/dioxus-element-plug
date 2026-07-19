use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str, fire_event};

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
    let class_string = ClassBuilder::new("el-dropdown")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
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
    let class_string = ClassBuilder::new("el-dropdown-menu__item")
        .add_if("is-disabled", props.disabled)
        .add_if("is-divided", props.divided)
        .add_if("is-selected", props.selected)
        .add_opt(props.class.as_ref())
        .build();

    let cmd_clone = props.command.clone();
    let on_click = props.on_click;
    let disabled = props.disabled;

    rsx! {
        li {
            class: "{class_string}",
            role: "menuitem",
            onclick: move |e| {
                if !disabled {
                    fire_event(&on_click, e);
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
    let class_string = ClassBuilder::new("el-dropdown-menu")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        ul {
            class: "{class_string}",
            style: "{style_string}",
            role: "menu",
            {props.children}
        }
    }
}
