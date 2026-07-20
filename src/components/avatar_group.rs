use crate::components::common::{style_str, ClassBuilder};
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
    let class_string = ClassBuilder::new("el-avatar-group")
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

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
