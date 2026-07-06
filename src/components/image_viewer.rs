use dioxus::prelude::*;

/// ImageViewer props
#[derive(Props, Clone, PartialEq)]
pub struct ImageViewerProps {
    /// Whether the viewer is visible
    #[props(default = false)]
    pub visible: bool,

    /// List of image URLs
    #[props(default)]
    pub url_list: Vec<String>,

    /// Initial image index
    #[props(default = 0)]
    pub initial_index: u32,

    /// Whether to show close button
    #[props(default = true)]
    pub show_close: bool,

    /// Close event handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// ImageViewer component for full-screen image preview
#[component]
pub fn ImageViewer(props: ImageViewerProps) -> Element {
    if !props.visible || props.url_list.is_empty() {
        return rsx! {};
    }

    let current_img = props.url_list.first().cloned().unwrap_or_default();

    rsx! {
        div {
            class: "el-image-viewer__wrapper",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2001;",
            div {
                class: "el-image-viewer__mask",
                style: "position: absolute; width: 100%; height: 100%; background: rgba(0,0,0,0.5);",
                onclick: move |_| {
                    if let Some(handler) = props.on_close {
                        handler.call(());
                    }
                },
            }
            div {
                class: "el-image-viewer__btn el-image-viewer__close",
                onclick: move |_| {
                    if let Some(handler) = props.on_close {
                        handler.call(());
                    }
                },
                "×"
            }
            img {
                class: "el-image-viewer__img",
                src: "{current_img}",
                style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); max-width: 100%; max-height: 100%;",
            }
        }
    }
}
