use dioxus::prelude::*;

/// AvatarGroup props
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Avatar group content
    #[props(default)]
    pub children: Element,

    /// Maximum number of avatars to display
    #[props(default)]
    pub max: Option<u32>,

    /// Size of avatars in the group
    #[props(default)]
    pub size: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// AvatarGroup component for displaying multiple avatars
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     AvatarGroup { max: Some(3),
///         Avatar { src: Some("user1.jpg".to_string()) }
///         Avatar { src: Some("user2.jpg".to_string()) }
///         Avatar { src: Some("user3.jpg".to_string()) }
///     }
/// }
/// ```
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let mut class_names = vec!["el-avatar-group".to_string()];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.clone());
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
            if let Some(max) = props.max {
                span {
                    class: "el-avatar el-avatar--more",
                    "+{max}"
                }
            }
        }
    }
}
