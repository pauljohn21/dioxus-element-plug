use dioxus::prelude::*;

/// Empty props - 空状态占位
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    /// Empty description text
    #[props(default)]
    pub description: Option<String>,

    /// Image URL
    #[props(default)]
    pub image: Option<String>,

    /// Image size (px)
    #[props(default)]
    pub image_size: Option<u32>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Empty component - placeholder for empty states
///
/// Mirrors Element Plus `el-empty`.
#[component]
pub fn Empty(props: EmptyProps) -> Element {
    let mut class_names = vec!["el-empty".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let image_style = props.image_size.map(|s| format!("width: {}px;", s)).unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            if let Some(ref img) = props.image {
                div {
                    class: "el-empty__image",
                    img { src: "{img}", style: "{image_style}" }
                }
            } else {
                div {
                    class: "el-empty__image",
                    style: "width: 160px; height: 160px; display: flex; align-items: center; justify-content: center; color: #dcdfe6;",
                    span { style: "font-size: 60px;", "∅" }
                }
            }

            if let Some(ref desc) = props.description {
                p {
                    class: "el-empty__description",
                    "{desc}"
                }
            }

            div {
                class: "el-empty__bottom",
            }
        }
    }
}
