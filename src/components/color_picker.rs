use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Convert HSV (h: 0-360, s: 0-100, v: 0-100) to a hex color string like `#RRGGBB`.
pub fn hsv_to_hex(h: u32, s: u32, v: u32) -> String {
    let h = h % 360;
    let s = (s.min(100) as f64) / 100.0;
    let v = (v.min(100) as f64) / 100.0;

    let c = v * s;
    let h_prime = (h as f64) / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r1 + m) * 255.0).round() as u32;
    let g = ((g1 + m) * 255.0).round() as u32;
    let b = ((b1 + m) * 255.0).round() as u32;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Convert a hex color string (`#RRGGBB` or `#RGB`) to HSV `(h: 0-360, s: 0-100, v: 0-100)`.
pub fn hex_to_hsv(hex: &str) -> (u32, u32, u32) {
    let hex = hex.trim_start_matches('#');
    let (r, g, b) = if hex.len() == 6 {
        let r = u32::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u32::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u32::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    } else if hex.len() == 3 {
        let r = u32::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
        let g = u32::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
        let b = u32::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
        (r, g, b)
    } else {
        (0, 0, 0)
    };

    let r_f = r as f64 / 255.0;
    let g_f = g as f64 / 255.0;
    let b_f = b as f64 / 255.0;

    let cmax = r_f.max(g_f).max(b_f);
    let cmin = r_f.min(g_f).min(b_f);
    let delta = cmax - cmin;

    let v = (cmax * 100.0).round() as u32;

    let s = if cmax == 0.0 {
        0
    } else {
        ((delta / cmax) * 100.0).round() as u32
    };

    let h: f64 = if delta == 0.0 {
        0.0
    } else if (cmax - r_f).abs() < f64::EPSILON {
        ((60.0 * (((g_f - b_f) / delta) % 6.0)) + 360.0) % 360.0
    } else if (cmax - g_f).abs() < f64::EPSILON {
        (60.0 * (((b_f - r_f) / delta) + 2.0)) % 360.0
    } else {
        (60.0 * (((r_f - g_f) / delta) + 4.0)) % 360.0
    };

    (h.round() as u32, s, v)
}

/// ColorPicker props
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    #[props(default = "#409EFF".to_string())]
    pub model_value: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub show_alpha: bool,

    #[props(default = "hex".to_string())]
    pub color_format: String,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// ColorPicker component for selecting colors
///
/// Renders a trigger swatch that opens an HSV-based selection panel.
/// The panel exposes a hue bar, an SV grid for the selected hue, a hex
/// input, and a row of common preset colors.
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let class_string = ClassBuilder::new("el-color-picker")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let mut panel_visible = use_signal(|| false);

    // Initialize HSV from the model_value.
    let (init_h, init_s, init_v) = hex_to_hsv(&props.model_value);
    let mut current_h = use_signal(|| init_h);
    let mut current_s = use_signal(|| init_s);
    let mut current_v = use_signal(|| init_v);
    let mut hex_input = use_signal(|| props.model_value.clone());

    // Sync internal state when model_value changes externally.
    use_effect(move || {
        let (h, s, v) = hex_to_hsv(&props.model_value);
        current_h.set(h);
        current_s.set(s);
        current_v.set(v);
        hex_input.set(props.model_value.clone());
    });

    let h = current_h();
    let s = current_s();
    let v = current_v();
    let current_hex = hsv_to_hex(h, s, v);
    let on_change = props.on_change;

    // Build the SV grid: 10 saturation steps × 8 value steps.
    let sv_cells: Vec<(u32, u32, String)> = {
        let mut cells = Vec::with_capacity(80);
        for vi in (0..=7).rev() {
            let vv = (vi as u32) * 100 / 7;
            for si in 0..=9 {
                let ss = (si as u32) * 100 / 9;
                cells.push((ss, vv, hsv_to_hex(h, ss, vv)));
            }
        }
        cells
    };

    // Hue segments for the hue bar (24 segments), pre-computed with style.
    let hue_segments: Vec<(u32, String, String)> = (0..24)
        .map(|i| {
            let hue = i * 15;
            let bg = hsv_to_hex(hue, 100, 100);
            let border = if hue == h { "#333".to_string() } else { "transparent".to_string() };
            (hue, bg, border)
        })
        .collect();

    // Common preset colors.
    let presets: &[&str] = &[
        "#45006E", "#7C4DFF", "#B388FF", "#03A9F4", "#1E88E5", "#00BCD4",
        "#4CAF50", "#8BC34A", "#CDDC39", "#FFEB3B", "#FFC107", "#FF9800",
        "#FF5722", "#795548", "#9E9E9E", "#607D8B", "#000000", "#FFFFFF",
    ];

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Trigger swatch
            div {
                class: "el-color-picker__trigger",
                style: "background-color: {current_hex};",
                onclick: move |_| {
                    if !props.disabled {
                        panel_visible.set(!panel_visible());
                    }
                },
            }

            if panel_visible() {
                div {
                    class: "el-color-picker__panel",
                    style: "position: absolute; z-index: 2000; padding: 10px; background: #fff; border: 1px solid #e4e7ed; border-radius: 4px; box-shadow: 0 2px 12px rgba(0,0,0,0.1);",

                    // SV grid
                    div {
                        class: "el-color-picker__sv-panel",
                        style: "display: grid; grid-template-columns: repeat(10, 1fr); gap: 2px; width: 200px; margin-bottom: 10px;",

                        for (ss, vv, hex) in sv_cells.into_iter() {
                            div {
                                class: "el-color-picker__sv-cell",
                                style: "height: 16px; background: {hex}; cursor: pointer; border: 1px solid transparent;",
                                onclick: move |_| {
                                    current_s.set(ss);
                                    current_v.set(vv);
                                    let new_hex = hsv_to_hex(current_h(), ss, vv);
                                    hex_input.set(new_hex.clone());
                                    fire_event(&on_change, new_hex);
                                },
                            }
                        }
                    }

                    // Hue bar
                    div {
                        class: "el-color-picker__hue-slider",
                        style: "display: flex; gap: 1px; width: 200px; height: 12px; margin-bottom: 10px;",

                        for (hue, bg, border) in hue_segments.into_iter() {
                            div {
                                class: "el-color-picker__hue-segment",
                                style: "flex: 1; background: {bg}; cursor: pointer; border: 1px solid {border};",
                                onclick: move |_| {
                                    current_h.set(hue);
                                    let new_hex = hsv_to_hex(hue, current_s(), current_v());
                                    hex_input.set(new_hex.clone());
                                    fire_event(&on_change, new_hex);
                                },
                            }
                        }
                    }

                    // Hex input + current preview
                    div {
                        class: "el-color-picker__input-group",
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 10px;",

                        div {
                            class: "el-color-picker__preview",
                            style: "width: 28px; height: 28px; border-radius: 4px; border: 1px solid #dcdfe6; background: {current_hex};",
                        }

                        input {
                            class: "el-input__inner",
                            r#type: "text",
                            style: "width: 100px; height: 28px; padding: 0 8px; font-size: 12px;",
                            value: "{hex_input}",
                            onchange: move |e: FormEvent| {
                                let val = e.value();
                                if val.starts_with('#') && (val.len() == 7 || val.len() == 4) {
                                    let (hh, hs, hv) = hex_to_hsv(&val);
                                    current_h.set(hh);
                                    current_s.set(hs);
                                    current_v.set(hv);
                                    hex_input.set(val.clone());
                                    fire_event(&on_change, val);
                                }
                            },
                        }
                    }

                    // Preset colors
                    div {
                        class: "el-color-picker__presets",
                        style: "display: flex; flex-wrap: wrap; gap: 4px; max-width: 200px;",

                        for preset in presets.iter() {
                            div {
                                class: "el-color-picker__preset",
                                style: "width: 20px; height: 20px; border-radius: 3px; cursor: pointer; border: 1px solid #dcdfe6; background: {preset};",
                                onclick: move |_| {
                                    let (hh, hs, hv) = hex_to_hsv(preset);
                                    current_h.set(hh);
                                    current_s.set(hs);
                                    current_v.set(hv);
                                    hex_input.set(preset.to_string());
                                    fire_event(&on_change, preset.to_string());
                                },
                            }
                        }
                    }

                    // Action buttons
                    div {
                        class: "el-color-picker__actions",
                        style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 10px;",

                        button {
                            class: "el-button el-button--default el-button--mini",
                            r#type: "button",
                            onclick: move |_| {
                                panel_visible.set(false);
                            },
                            "取消"
                        }

                        button {
                            class: "el-button el-button--primary el-button--mini",
                            r#type: "button",
                            onclick: move |_| {
                                let final_hex = hsv_to_hex(current_h(), current_s(), current_v());
                                fire_event(&on_change, final_hex);
                                panel_visible.set(false);
                            },
                            "确定"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_hex_red() {
        // HSV(0, 100%, 100%) = pure red
        assert_eq!(hsv_to_hex(0, 100, 100), "#FF0000");
    }

    #[test]
    fn test_hsv_to_hex_green() {
        // HSV(120, 100%, 100%) = pure green
        assert_eq!(hsv_to_hex(120, 100, 100), "#00FF00");
    }

    #[test]
    fn test_hsv_to_hex_blue() {
        // HSV(240, 100%, 100%) = pure blue
        assert_eq!(hsv_to_hex(240, 100, 100), "#0000FF");
    }

    #[test]
    fn test_hsv_to_hex_yellow() {
        // HSV(60, 100%, 100%) = yellow
        assert_eq!(hsv_to_hex(60, 100, 100), "#FFFF00");
    }

    #[test]
    fn test_hsv_to_hex_white() {
        // HSV(any, 0%, 100%) = white
        assert_eq!(hsv_to_hex(0, 0, 100), "#FFFFFF");
    }

    #[test]
    fn test_hsv_to_hex_black() {
        // HSV(any, any, 0%) = black
        assert_eq!(hsv_to_hex(0, 100, 0), "#000000");
    }

    #[test]
    fn test_hsv_to_hex_element_blue() {
        // Element Plus primary #409EFF ≈ HSV(210, 75%, 100%).
        // The HSV→HEX round-trip introduces a tiny rounding drift on the
        // green channel (159 vs 158), which is expected for discrete HSV.
        assert_eq!(hsv_to_hex(210, 75, 100), "#409FFF");
    }

    #[test]
    fn test_hex_to_hsv_red() {
        let (h, s, v) = hex_to_hsv("#FF0000");
        assert_eq!(h, 0);
        assert_eq!(s, 100);
        assert_eq!(v, 100);
    }

    #[test]
    fn test_hex_to_hsv_green() {
        let (h, s, v) = hex_to_hsv("#00FF00");
        assert_eq!(h, 120);
        assert_eq!(s, 100);
        assert_eq!(v, 100);
    }

    #[test]
    fn test_hex_to_hsv_blue() {
        let (h, s, v) = hex_to_hsv("#0000FF");
        assert_eq!(h, 240);
        assert_eq!(s, 100);
        assert_eq!(v, 100);
    }

    #[test]
    fn test_hex_to_hsv_white() {
        let (_h, s, v) = hex_to_hsv("#FFFFFF");
        assert_eq!(s, 0);
        assert_eq!(v, 100);
    }

    #[test]
    fn test_hex_to_hsv_black() {
        let (_h, s, v) = hex_to_hsv("#000000");
        assert_eq!(s, 0);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_hex_to_hsv_short_format() {
        // #F00 → red
        let (h, s, v) = hex_to_hsv("#F00");
        assert_eq!(h, 0);
        assert_eq!(s, 100);
        assert_eq!(v, 100);
    }

    #[test]
    fn test_hsv_hex_roundtrip() {
        // Round-trip: hex → hsv → hex should be identity for common colors.
        let test_colors = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF", "#00FFFF"];
        for original in test_colors.iter() {
            let (h, s, v) = hex_to_hsv(original);
            let result = hsv_to_hex(h, s, v);
            assert_eq!(&result, original, "round-trip failed for {}", original);
        }
    }
}
