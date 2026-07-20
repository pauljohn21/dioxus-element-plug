use crate::components::common::{fire_event, style_str, ClassBuilder};
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

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// ImageViewer component for full-screen image preview with zoom and rotation
#[component]
pub fn ImageViewer(props: ImageViewerProps) -> Element {
    let url_count = props.url_list.len() as u32;

    let mut current_index = use_signal(|| props.initial_index.min(url_count.saturating_sub(1)));
    let mut zoom = use_signal(|| 1.0f32);
    let mut rotation = use_signal(|| 0i32);

    // Sync index when initial_index or url_list changes
    if current_index() >= url_count && url_count > 0 {
        current_index.set(url_count - 1);
    }

    if !props.visible || props.url_list.is_empty() {
        return rsx! {};
    }

    let idx = current_index().min(url_count.saturating_sub(1));
    let current_img = props
        .url_list
        .get(idx as usize)
        .cloned()
        .unwrap_or_default();
    let z = zoom();
    let r = rotation();

    let class_string = ClassBuilder::new("el-image-viewer__wrapper")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_close = props.on_close;

    let img_style = format!(
        "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%) scale({z}) rotate({r}deg); \
        max-width: 80%; max-height: 80%; transition: transform 0.3s ease;",
        z = z,
        r = r,
    );

    rsx! {
        div {
            class: "{class_string}",
            style: "position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 2001; {style_string}",
            // Mask
            div {
                class: "el-image-viewer__mask",
                style: "position: absolute; width: 100%; height: 100%; background: rgba(0,0,0,0.5);",
                onclick: move |_| {
                    fire_event(&on_close, ());
                },
            }
            // Close button
            if props.show_close {
                div {
                    class: "el-image-viewer__btn el-image-viewer__close",
                    style: "position: absolute; top: 40px; right: 40px; font-size: 40px; color: #fff; cursor: pointer; z-index: 1;",
                    onclick: move |_| {
                        fire_event(&on_close, ());
                    },
                    "×"
                }
            }
            // Prev button
            if url_count > 1 {
                div {
                    class: "el-image-viewer__btn el-image-viewer__prev",
                    style: "position: absolute; top: 50%; left: 40px; transform: translateY(-50%); font-size: 40px; color: #fff; cursor: pointer; z-index: 1;",
                    onclick: move |_| {
                        let new_idx = if current_index() == 0 {
                            url_count - 1
                        } else {
                            current_index() - 1
                        };
                        current_index.set(new_idx);
                        zoom.set(1.0);
                        rotation.set(0);
                    },
                    "‹"
                }
                // Next button
                div {
                    class: "el-image-viewer__btn el-image-viewer__next",
                    style: "position: absolute; top: 50%; right: 40px; transform: translateY(-50%); font-size: 40px; color: #fff; cursor: pointer; z-index: 1;",
                    onclick: move |_| {
                        let new_idx = (current_index() + 1) % url_count;
                        current_index.set(new_idx);
                        zoom.set(1.0);
                        rotation.set(0);
                    },
                    "›"
                }
            }
            // Image
            img {
                class: "el-image-viewer__img",
                src: "{current_img}",
                style: "{img_style}",
            }
            // Zoom and rotation controls
            if url_count > 0 {
                div {
                    class: "el-image-viewer__actions",
                    style: "position: absolute; bottom: 30px; left: 50%; transform: translateX(-50%); display: flex; gap: 10px; background: rgba(0,0,0,0.5); border-radius: 22px; padding: 0 16px; height: 44px; align-items: center; color: #fff;",
                    // Zoom out
                    button {
                        style: "background: none; border: none; color: #fff; cursor: pointer; font-size: 20px;",
                        onclick: move |_| {
                            let new_zoom = (zoom() - 0.2).max(0.2);
                            zoom.set(new_zoom);
                        },
                        "−"
                    }
                    // Zoom reset
                    button {
                        style: "background: none; border: none; color: #fff; cursor: pointer; font-size: 14px;",
                        onclick: move |_| {
                            zoom.set(1.0);
                            rotation.set(0);
                        },
                        "{z:.1}x"
                    }
                    // Zoom in
                    button {
                        style: "background: none; border: none; color: #fff; cursor: pointer; font-size: 20px;",
                        onclick: move |_| {
                            let new_zoom = (zoom() + 0.2).min(5.0);
                            zoom.set(new_zoom);
                        },
                        "+"
                    }
                    // Separator
                    span { style: "color: #fff; opacity: 0.5;", "|" }
                    // Rotate
                    button {
                        style: "background: none; border: none; color: #fff; cursor: pointer; font-size: 18px;",
                        onclick: move |_| {
                            rotation.set(rotation() + 90);
                        },
                        "↻"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zoom_bounds() {
        // Zoom out minimum is 0.2
        let mut z: f32 = 1.0;
        z = (z - 0.2).max(0.2);
        assert!((z - 0.8).abs() < 0.001);
        z = (z - 0.2).max(0.2);
        assert!((z - 0.6).abs() < 0.001);
        z = (z - 0.2).max(0.2);
        assert!((z - 0.4).abs() < 0.001);
        z = (z - 0.2).max(0.2);
        assert!((z - 0.2).abs() < 0.001);
        // Should not go below 0.2
        z = (z - 0.2).max(0.2);
        assert!((z - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_zoom_in_max() {
        let mut z: f32 = 1.0;
        z = (z + 0.2).min(5.0);
        assert_eq!(z, 1.2);
        // Simulate many zoom ins
        for _ in 0..100 {
            z = (z + 0.2).min(5.0);
        }
        assert_eq!(z, 5.0);
    }

    #[test]
    fn test_rotation_increment() {
        let mut r: i32 = 0;
        r += 90;
        assert_eq!(r, 90);
        r += 90;
        assert_eq!(r, 180);
        r += 90;
        assert_eq!(r, 270);
        r += 90;
        assert_eq!(r, 360);
    }

    #[test]
    fn test_index_wrap_around() {
        let url_count: u32 = 3;
        let mut idx: u32 = 2;
        // Next wraps to 0
        idx = (idx + 1) % url_count;
        assert_eq!(idx, 0);
        // Prev from 0 wraps to 2
        idx = if idx == 0 { url_count - 1 } else { idx - 1 };
        assert_eq!(idx, 2);
    }
}
