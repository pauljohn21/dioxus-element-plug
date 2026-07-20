use crate::components::common::{fire_event, style_str, ClassBuilder};
use crate::components::image_viewer::ImageViewer;
use dioxus::prelude::*;

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

    /// Whether to enable preview on click
    #[props(default = true)]
    pub preview: bool,

    /// Placeholder content while loading
    #[props(default)]
    pub placeholder: Option<String>,

    /// Error content
    #[props(default)]
    pub error: Option<String>,

    /// Image load event handler
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Image error event handler
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

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
    let mut preview_visible = use_signal(|| false);
    let mut is_loading = use_signal(|| true);
    let mut has_error = use_signal(|| false);

    let class_string = ClassBuilder::new("el-image")
        .add_if("is-loading", is_loading())
        .add_if("is-error", has_error())
        .add_opt(props.class.as_ref())
        .build();
    let fit_str = props.fit.as_str();
    let style_string = style_str(&props.style);

    let on_load = props.on_load;
    let on_error = props.on_error;

    // Determine preview list
    let preview_urls: Vec<String> = if props.preview_src_list.is_empty() {
        vec![props.src.clone()]
    } else {
        props.preview_src_list.clone()
    };

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
                onclick: move |_| {
                    if props.preview {
                        preview_visible.set(true);
                    }
                },
                onload: move |_| {
                    is_loading.set(false);
                    fire_event(&on_load, ());
                },
                onerror: move |_| {
                    is_loading.set(false);
                    has_error.set(true);
                    fire_event(&on_error, ());
                },
            }
            // Loading placeholder
            if is_loading() {
                div {
                    class: "el-image__placeholder",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; background: #f5f7fa;",
                    {props.placeholder.as_deref().unwrap_or("Loading...")}
                }
            }
            // Error state
            if has_error() {
                div {
                    class: "el-image__error",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; background: #f5f7fa; color: #909399;",
                    {props.error.as_deref().unwrap_or("Load failed")}
                }
            }
            // Preview viewer
            if preview_visible() {
                ImageViewer {
                    visible: preview_visible(),
                    url_list: preview_urls.clone(),
                    initial_index: 0,
                    on_close: move |_| preview_visible.set(false),
                }
            }
        }
    }
}
