use dioxus::prelude::*;

/// Avatar size
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

/// Avatar shape
#[derive(Clone, PartialEq)]
pub enum AvatarShape {
    Circle,
    Square,
}

/// Avatar props - 头像
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Avatar content (text or image)
    #[props(default)]
    pub children: Element,

    /// Size
    #[props(default = AvatarSize::Default)]
    pub size: AvatarSize,

    /// Shape
    #[props(default = AvatarShape::Circle)]
    pub shape: AvatarShape,

    /// Icon class
    #[props(default)]
    pub icon: Option<String>,

    /// Image source
    #[props(default)]
    pub src: Option<String>,

    /// Fit mode
    #[props(default = "cover".to_string())]
    pub fit: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Avatar component
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let mut class_names = vec!["el-avatar".to_string()];
    class_names.push(props.size.as_class().to_string());
    if matches!(props.shape, AvatarShape::Square) {
        class_names.push("el-avatar--square".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        span {
            class: "{class_string}",
            style: "{style_string}",

            if let Some(ref src) = props.src {
                img { src: "{src}", style: "object-fit: {props.fit};" }
            } else if let Some(ref icon) = props.icon {
                i { class: "{icon}" }
            } else {
                {props.children}
            }
        }
    }
}
