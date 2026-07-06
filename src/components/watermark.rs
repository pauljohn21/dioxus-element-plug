use dioxus::prelude::*;

/// Watermark props
#[derive(Props, Clone, PartialEq)]
pub struct WatermarkProps {
    #[props(default)]
    pub children: Element,

    /// Watermark text
    pub content: String,

    /// Font size
    #[props(default = 16)]
    pub font_size: u32,

    /// Font color
    #[props(default = "rgba(0,0,0,0.15)".to_string())]
    pub font_color: String,

    /// Gap between watermarks (X)
    #[props(default = 100)]
    pub gap_x: u32,

    /// Gap between watermarks (Y)
    #[props(default = 100)]
    pub gap_y: u32,

    /// Rotation angle
    #[props(default = -22)]
    pub rotate: i32,

    /// Z-index
    #[props(default = 9)]
    pub z_index: u32,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Watermark component for overlaying watermark text
#[component]
pub fn Watermark(props: WatermarkProps) -> Element {
    let watermark_style = format!(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: {}; \
        background-image: repeating-linear-gradient({}deg, transparent, transparent {}px, transparent {}px), \
        repeating-linear-gradient(0deg, transparent, transparent {}px, transparent {}px); \
        pointer-events: none; opacity: 0.5;",
        props.z_index, props.rotate, props.gap_y, props.gap_y, props.gap_x, props.gap_x
    );

    let font_size = props.font_size;
    let font_color = props.font_color.clone();

    rsx! {
        div {
            class: "el-watermark {props.class.clone().unwrap_or_default()}",
            style: "position: relative; {props.style.clone().unwrap_or_default()}",
            {props.children}
            div {
                class: "el-watermark__overlay",
                style: "{watermark_style}",
                span {
                    style: "font-size: {font_size}px; color: {font_color}; opacity: 0.3;",
                    "{props.content}"
                }
            }
        }
    }
}
