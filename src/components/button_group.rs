use dioxus::prelude::*;

/// ButtonGroup props - 按钮组
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    pub children: Element,

    #[props(default)]
    pub size: Option<super::button::ButtonSize>,

    #[props(default)]
    pub r#type: Option<super::button::ButtonVariant>,

    #[props(default)]
    pub class: Option<String>,
}

/// ButtonGroup component
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let mut class_names = vec!["el-button-group".to_string()];
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
