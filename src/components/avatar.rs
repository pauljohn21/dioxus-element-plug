use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Avatar size variants
#[derive(Clone, PartialEq)]
pub enum AvatarSize {
    Large,
    Default,
    Small,
}

impl AvatarSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            AvatarSize::Large => "el-avatar--large",
            AvatarSize::Default => "",
            AvatarSize::Small => "el-avatar--small",
        }
    }
}

/// Avatar shape variants
#[derive(Clone, PartialEq)]
pub enum AvatarShape {
    Circle,
    Square,
}

impl AvatarShape {
    pub fn as_class(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "el-avatar--circle",
            AvatarShape::Square => "el-avatar--square",
        }
    }
}

/// Avatar fit type
#[derive(Clone, PartialEq)]
pub enum AvatarFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

impl AvatarFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarFit::Fill => "fill",
            AvatarFit::Contain => "contain",
            AvatarFit::Cover => "cover",
            AvatarFit::None => "none",
            AvatarFit::ScaleDown => "scale-down",
        }
    }
}

/// Avatar props
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Avatar content (text or icon)
    #[props(default)]
    pub children: Option<Element>,

    /// Image source URL
    #[props(default)]
    pub src: Option<String>,

    /// Icon CSS class
    #[props(default)]
    pub icon: Option<String>,

    /// Avatar size
    #[props(default = AvatarSize::Default)]
    pub size: AvatarSize,

    /// Avatar shape
    #[props(default = AvatarShape::Circle)]
    pub shape: AvatarShape,

    /// Image fit type
    #[props(default = AvatarFit::Cover)]
    pub fit: AvatarFit,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Avatar component for displaying user profiles or icons
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Avatar { src: Some("user.jpg".to_string()), size: AvatarSize::Large }
///     Avatar { "U" }
/// }
/// ```
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class_string = ClassBuilder::new("el-avatar")
        .add_class(props.shape.as_class())
        .add_class(props.size.as_class())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let fit_str = props.fit.as_str();

    rsx! {
        span {
            class: "{class_string}",
            style: "{style_string}",
            if let Some(ref src) = props.src {
                img {
                    class: "el-avatar__img",
                    src: "{src}",
                    style: "object-fit: {fit_str};",
                }
            } else if let Some(ref icon) = props.icon {
                i {
                    class: "{icon}"
                }
            } else {
                {props.children}
            }
        }
    }
}
