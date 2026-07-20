use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// Generate an SVG watermark string with tiled text pattern.
///
/// Creates an SVG with a `<pattern>` element that tiles the watermark text
/// at the specified gap intervals and rotation angle.
fn generate_watermark_svg(
    content: &str,
    font_size: u32,
    font_color: &str,
    gap_x: u32,
    gap_y: u32,
    rotate: i32,
) -> String {
    // Escape XML special characters in content
    let escaped_content = content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    let tile_width = gap_x.max(font_size);
    let tile_height = gap_y.max(font_size);

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{tw}" height="{th}"><defs><pattern id="wm" patternUnits="userSpaceOnUse" width="{gx}" height="{gy}"><text x="0" y="{fs}" font-size="{fs}" fill="{fc}" transform="rotate({rot},0,{fs})">{txt}</text></pattern></defs><rect width="100%" height="100%" fill="url(#wm)"/></svg>"#,
        tw = tile_width,
        th = tile_height,
        gx = gap_x,
        gy = gap_y,
        fs = font_size,
        fc = font_color,
        rot = rotate,
        txt = escaped_content,
    )
}

/// Encode an SVG string as a CSS `url()` data URI.
fn svg_to_data_uri(svg: &str) -> String {
    let encoded = svg.replace('#', "%23").replace('"', "'").replace('\n', "");
    format!("url(\"data:image/svg+xml,{encoded}\")")
}

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

/// Watermark component for overlaying tiled watermark text
#[component]
pub fn Watermark(props: WatermarkProps) -> Element {
    let svg = generate_watermark_svg(
        &props.content,
        props.font_size,
        &props.font_color,
        props.gap_x,
        props.gap_y,
        props.rotate,
    );
    let bg_image = svg_to_data_uri(&svg);

    let class_string = ClassBuilder::new("el-watermark")
        .add_opt(props.class.as_ref())
        .build();

    let container_style = format!("position: relative; {}", style_str(&props.style));

    let overlay_style = format!(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; \
        z-index: {z}; background-image: {bg}; pointer-events: none;",
        z = props.z_index,
        bg = bg_image,
    );

    rsx! {
        div {
            class: "{class_string}",
            style: "{container_style}",
            {props.children}
            div {
                class: "el-watermark__overlay",
                style: "{overlay_style}",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_watermark_svg_contains_text() {
        let svg = generate_watermark_svg("CONFIDENTIAL", 16, "rgba(0,0,0,0.15)", 100, 100, -22);
        assert!(svg.contains("CONFIDENTIAL"));
        assert!(svg.contains("font-size=\"16\""));
        assert!(svg.contains("fill=\"rgba(0,0,0,0.15)\""));
        assert!(svg.contains("rotate(-22"));
        assert!(svg.contains("<pattern"));
    }

    #[test]
    fn test_generate_watermark_svg_escapes_special_chars() {
        let svg = generate_watermark_svg("<test> & \"quote\"", 14, "#000", 80, 80, 0);
        assert!(svg.contains("&lt;test&gt;"));
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&quot;"));
    }

    #[test]
    fn test_svg_to_data_uri() {
        // SVG with # character to test encoding
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#000000"></svg>"##;
        let uri = svg_to_data_uri(svg);
        assert!(uri.starts_with("url(\"data:image/svg+xml,"));
        assert!(uri.ends_with("\")"));
        assert!(uri.contains("%23")); // # should be encoded
    }
}
