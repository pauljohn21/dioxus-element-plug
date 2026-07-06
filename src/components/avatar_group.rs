use dioxus::prelude::*;

/// AvatarGroup props - 头像组
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    pub children: Element,

    #[props(default = 0)]
    pub max: u32,

    #[props(default)]
    pub size: Option<super::avatar::AvatarSize>,

    #[props(default)]
    pub class: Option<String>,
}

/// AvatarGroup component
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let mut class_names = vec!["el-avatar-group".to_string()];
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
