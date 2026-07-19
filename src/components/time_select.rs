use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Parse a "HH:MM" time string into total minutes.
fn parse_time(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let hours: u32 = parts[0].parse().ok()?;
    let minutes: u32 = parts[1].parse().ok()?;
    Some(hours * 60 + minutes)
}

/// Format total minutes as "HH:MM".
fn format_time(minutes: u32) -> String {
    let h = minutes / 60;
    let m = minutes % 60;
    format!("{:02}:{:02}", h, m)
}

/// Generate a list of time options from `start` to `end` with `step` intervals.
///
/// All parameters are in "HH:MM" format. Returns an empty Vec if `step` is
/// zero or `start` is greater than `end`.
fn generate_time_options(start: &str, end: &str, step: &str) -> Vec<String> {
    let start_min = parse_time(start).unwrap_or(0);
    let end_min = parse_time(end).unwrap_or(0);
    let step_min = parse_time(step).unwrap_or(30);

    if step_min == 0 || start_min > end_min {
        return vec![];
    }

    let mut result = vec![];
    let mut current = start_min;
    while current <= end_min {
        result.push(format_time(current));
        current += step_min;
    }
    result
}

/// TimeSelect props
#[derive(Props, Clone, PartialEq)]
pub struct TimeSelectProps {
    #[props(default)]
    pub model_value: Option<String>,

    #[props(default = "Select Time".to_string())]
    pub placeholder: String,

    #[props(default = "00:00".to_string())]
    pub start: String,

    #[props(default = "23:59".to_string())]
    pub end: String,

    #[props(default = "00:30".to_string())]
    pub step: String,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default = false)]
    pub clearable: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// TimeSelect component for fixed-time selection
#[component]
pub fn TimeSelect(props: TimeSelectProps) -> Element {
    let mut dropdown_open = use_signal(|| false);

    let class_string = ClassBuilder::new("el-time-select")
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let options = generate_time_options(&props.start, &props.end, &props.step);
    let display_value = props.model_value.clone().unwrap_or_default();
    let on_change = props.on_change;
    let disabled = props.disabled;
    let clearable = props.clearable;
    let placeholder = props.placeholder.clone();
    let has_value = props.model_value.is_some();

    // Pre-extract option data as owned tuples to avoid lifetime issues in closures
    let option_data: Vec<(String, String, bool)> = options
        .iter()
        .map(|o| {
            let is_selected = props.model_value.as_deref() == Some(o.as_str());
            (o.clone(), o.clone(), is_selected)
        })
        .collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-input el-input--default el-input--suffix",
                div {
                    class: "el-input__wrapper",
                    onclick: move |_| {
                        if !disabled {
                            dropdown_open.set(!dropdown_open());
                        }
                    },
                    input {
                        class: "el-input__inner",
                        r#type: "text",
                        placeholder: "{placeholder}",
                        value: "{display_value}",
                        disabled: disabled,
                        readonly: true,
                    }
                    span {
                        class: "el-input__suffix",
                        if clearable && has_value {
                            i {
                                class: "el-input__icon el-icon-circle-close",
                                onclick: move |e: MouseEvent| {
                                    e.stop_propagation();
                                    fire_event(&on_change, String::new());
                                    dropdown_open.set(false);
                                },
                            }
                        } else {
                            i { class: "el-input__icon el-icon-arrow-down" }
                        }
                    }
                }
            }
            if dropdown_open() && !disabled {
                div {
                    class: "el-select__popper el-popper",
                    style: "position: absolute; z-index: 2001;",
                    div {
                        class: "el-select-dropdown",
                        for (opt_value, opt_label, is_selected) in option_data.into_iter() {
                            div {
                                class: "el-select-dropdown__item",
                                class: if is_selected { "selected" },
                                onclick: move |_| {
                                    fire_event(&on_change, opt_value.clone());
                                    dropdown_open.set(false);
                                },
                                "{opt_label}"
                            }
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
    fn test_generate_time_options_normal() {
        let options = generate_time_options("08:00", "10:00", "00:30");
        assert_eq!(options, vec!["08:00", "08:30", "09:00", "09:30", "10:00"]);
    }

    #[test]
    fn test_generate_time_options_step_zero() {
        let options = generate_time_options("08:00", "10:00", "00:00");
        assert!(options.is_empty());
    }

    #[test]
    fn test_generate_time_options_start_after_end() {
        let options = generate_time_options("10:00", "08:00", "00:30");
        assert!(options.is_empty());
    }

    #[test]
    fn test_generate_time_options_hour_step() {
        let options = generate_time_options("09:00", "12:00", "01:00");
        assert_eq!(options, vec!["09:00", "10:00", "11:00", "12:00"]);
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("08:30"), Some(510));
        assert_eq!(parse_time("00:00"), Some(0));
        assert_eq!(parse_time("23:59"), Some(1439));
        assert_eq!(parse_time("invalid"), None);
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(510), "08:30");
        assert_eq!(format_time(0), "00:00");
        assert_eq!(format_time(1439), "23:59");
    }
}
