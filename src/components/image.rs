use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Image fit type
#[derive(Clone, PartialEq)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

impl ImageFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageFit::Fill => "fill",
            ImageFit::Contain => "contain",
            ImageFit::Cover => "cover",
            ImageFit::None => "none",
            ImageFit::ScaleDown => "scale-down",
        }
    }
}

/// Image props
#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    /// Image source URL
    pub src: String,

    /// Alt text
    #[props(default)]
    pub alt: Option<String>,

    /// Image fit type
    #[props(default = ImageFit::Cover)]
    pub fit: ImageFit,

    /// Whether to lazy load
    #[props(default = false)]
    pub lazy: bool,

    /// Preview src list
    #[props(default)]
    pub preview_src_list: Vec<String>,

    /// Whether to hide click handler for preview
    #[props(default = true)]
    pub preview: bool,

    /// Placeholder content while loading
    #[props(default)]
    pub placeholder: Option<String>,

    /// Error content
    #[props(default)]
    pub error: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Image component with preview support
#[component]
pub fn Image(props: ImageProps) -> Element {
    let class_string = ClassBuilder::new("el-image")
        .add_opt(props.class.as_ref())
        .build();
    let fit_str = props.fit.as_str();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            img {
                class: "el-image__inner",
                src: "{props.src}",
                alt: props.alt.clone().unwrap_or_default(),
                style: "object-fit: {fit_str};",
                loading: if props.lazy { "lazy" } else { "eager" },
            }
        }
    }
}
