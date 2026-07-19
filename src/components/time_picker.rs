use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};
use std::fmt;

/// TimePicker size variants
#[derive(Clone, PartialEq, Default)]
pub enum TimePickerSize {
    #[default]
    Default,
    Large,
    Small,
}

impl TimePickerSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            TimePickerSize::Default => "",
            TimePickerSize::Large => "el-time-editor--large",
            TimePickerSize::Small => "el-time-editor--small",
        }
    }
}

/// Time value structure
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TimeValue {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl TimeValue {
    pub fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self { hour, minute, second }
    }

    pub fn from_string(time_str: &str) -> Option<Self> {
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() < 2 {
            return None;
        }
        let hour = parts[0].parse::<u32>().ok()?;
        let minute = parts[1].parse::<u32>().ok()?;
        let second = if parts.len() > 2 {
            parts[2].parse::<u32>().ok()?
        } else {
            0
        };
        Some(Self::new(hour, minute, second))
    }

    pub fn to_short_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
}

impl fmt::Display for TimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

/// TimePicker props
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Selected time value (format: HH:mm:ss or HH:mm)
    #[props(default)]
    pub model_value: Option<String>,

    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,

    /// Time format
    #[props(default = "HH:mm:ss".to_string())]
    pub format: String,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether clearable
    #[props(default = false)]
    pub clearable: bool,

    /// Whether readonly
    #[props(default = false)]
    pub readonly: bool,

    /// Size variant
    #[props(default = TimePickerSize::Default)]
    pub size: TimePickerSize,

    /// Whether to show seconds
    #[props(default = true)]
    pub show_seconds: bool,

    /// Minimum selectable time
    #[props(default)]
    pub min_time: Option<String>,

    /// Maximum selectable time
    #[props(default)]
    pub max_time: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Focus event handler
    #[props(default)]
    pub on_focus: Option<EventHandler<()>>,

    /// Blur event handler
    #[props(default)]
    pub on_blur: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// TimePicker component for time selection
///
/// Provides a time input with scrollable time selection panel.
/// Supports hour, minute, and second selection.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::time_picker::TimePicker;
///
/// rsx! {
///     TimePicker {
///         placeholder: Some("选择时间".to_string()),
///         show_seconds: true,
///         on_change: move |time: String| println!("Selected: {}", time),
///     }
/// }
/// ```
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let mut panel_visible = use_signal(|| false);
    let mut input_value = use_signal(|| props.model_value.clone().unwrap_or_default());
    let mut selected_hour = use_signal(|| 0u32);
    let mut selected_minute = use_signal(|| 0u32);
    let mut selected_second = use_signal(|| 0u32);

    // Initialize from model_value
    use_effect(move || {
        if let Some(ref value) = props.model_value {
            input_value.set(value.clone());
            if let Some(time) = TimeValue::from_string(value) {
                selected_hour.set(time.hour);
                selected_minute.set(time.minute);
                selected_second.set(time.second);
            }
        }
    });

    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "选择时间".to_string());

    // Build class string
    let class_string = ClassBuilder::new("el-date-editor el-time-editor")
        .add_class("el-time-editor--time")
        .add_class(props.size.as_class())
        .add_if("is-disabled", props.disabled)
        .add_if("is-active", panel_visible())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let on_change = props.on_change;
    let on_focus = props.on_focus;
    let on_blur = props.on_blur;

    // Generate time lists
    let hours: Vec<u32> = (0..24).collect();
    let minutes: Vec<u32> = (0..60).collect();
    let seconds: Vec<u32> = (0..60).collect();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Input wrapper
            div {
                class: "el-input el-input--default",
                onclick: move |_| {
                    if !props.disabled && !props.readonly {
                        panel_visible.set(true);
                        fire_event(&on_focus, ());
                    }
                },

                // Input element
                input {
                    class: "el-input__inner",
                    r#type: "text",
                    placeholder: "{placeholder}",
                    value: "{input_value}",
                    disabled: props.disabled,
                    readonly: props.readonly,
                    onchange: move |e: FormEvent| {
                        let value = e.value();
                        input_value.set(value.clone());
                        fire_event(&on_change, value);
                    },
                }

                // Prefix icon
                span {
                    class: "el-input__prefix",
                    i { class: "el-icon-time" }
                }

                // Clear button
                if props.clearable && !input_value().is_empty() && !props.disabled {
                    span {
                        class: "el-input__suffix",
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            input_value.set(String::new());
                            fire_event(&on_change, String::new());
                        },
                        i { class: "el-icon-circle-close el-input__clear" }
                    }
                }
            }

            // Time picker panel
            if panel_visible() {
                div {
                    class: "el-time-panel",
                    onclick: move |e: MouseEvent| e.stop_propagation(),

                    // Time spinner
                    div {
                        class: "el-time-spinner",

                        // Hours column
                        div {
                            class: "el-time-spinner__wrapper",
                            ul {
                                class: "el-time-spinner__list",
                                for hour in hours {
                                    {
                                        let is_selected = selected_hour() == hour;
                                        let is_disabled = is_time_disabled(hour, selected_minute(), selected_second(), &props.min_time, &props.max_time);
                                        let class_name = if is_selected {
                                            "el-time-spinner__item active"
                                        } else if is_disabled {
                                            "el-time-spinner__item disabled"
                                        } else {
                                            "el-time-spinner__item"
                                        };
                                        {
                                            let hour_str = format!("{:02}", hour);
                                            rsx! {
                                                li {
                                                    class: "{class_name}",
                                                    onclick: move |_| {
                                                        if !is_disabled {
                                                            selected_hour.set(hour);
                                                        }
                                                    },
                                                    "{hour_str}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Minutes column
                        div {
                            class: "el-time-spinner__wrapper",
                            ul {
                                class: "el-time-spinner__list",
                                for minute in minutes {
                                    {
                                        let is_selected = selected_minute() == minute;
                                        let is_disabled = is_time_disabled(selected_hour(), minute, selected_second(), &props.min_time, &props.max_time);
                                        let class_name = if is_selected {
                                            "el-time-spinner__item active"
                                        } else if is_disabled {
                                            "el-time-spinner__item disabled"
                                        } else {
                                            "el-time-spinner__item"
                                        };
                                        {
                                            let minute_str = format!("{:02}", minute);
                                            rsx! {
                                                li {
                                                    class: "{class_name}",
                                                    onclick: move |_| {
                                                        if !is_disabled {
                                                            selected_minute.set(minute);
                                                        }
                                                    },
                                                    "{minute_str}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Seconds column
                        if props.show_seconds {
                            div {
                                class: "el-time-spinner__wrapper",
                                ul {
                                    class: "el-time-spinner__list",
                                    for second in seconds {
                                        {
                                            let is_selected = selected_second() == second;
                                            let is_disabled = is_time_disabled(selected_hour(), selected_minute(), second, &props.min_time, &props.max_time);
                                            let class_name = if is_selected {
                                                "el-time-spinner__item active"
                                            } else if is_disabled {
                                                "el-time-spinner__item disabled"
                                            } else {
                                                "el-time-spinner__item"
                                            };
                                            {
                                                let second_str = format!("{:02}", second);
                                                rsx! {
                                                    li {
                                                        class: "{class_name}",
                                                        onclick: move |_| {
                                                            if !is_disabled {
                                                                selected_second.set(second);
                                                            }
                                                        },
                                                        "{second_str}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Panel footer
                    div {
                        class: "el-time-panel__footer",
                        button {
                            class: "el-time-panel__btn cancel",
                            onclick: move |_| {
                                panel_visible.set(false);
                                fire_event(&on_blur, ());
                            },
                            "取消"
                        }
                        button {
                            class: "el-time-panel__btn confirm",
                            onclick: move |_| {
                                let time = if props.show_seconds {
                                    format!("{:02}:{:02}:{:02}", selected_hour(), selected_minute(), selected_second())
                                } else {
                                    format!("{:02}:{:02}", selected_hour(), selected_minute())
                                };
                                input_value.set(time.clone());
                                fire_event(&on_change, time);
                                panel_visible.set(false);
                                fire_event(&on_blur, ());
                            },
                            "确定"
                        }
                    }
                }
            }
        }
    }
}

/// Check if time is disabled
fn is_time_disabled(hour: u32, minute: u32, second: u32, min_time: &Option<String>, max_time: &Option<String>) -> bool {
    let current_seconds = hour * 3600 + minute * 60 + second;

    if let Some(ref min) = min_time {
        if let Some(min_t) = TimeValue::from_string(min) {
            let min_seconds = min_t.hour * 3600 + min_t.minute * 60 + min_t.second;
            if current_seconds < min_seconds {
                return true;
            }
        }
    }

    if let Some(ref max) = max_time {
        if let Some(max_t) = TimeValue::from_string(max) {
            let max_seconds = max_t.hour * 3600 + max_t.minute * 60 + max_t.second;
            if current_seconds > max_seconds {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_picker_size_as_class() {
        assert_eq!(TimePickerSize::Default.as_class(), "");
        assert_eq!(TimePickerSize::Large.as_class(), "el-time-editor--large");
        assert_eq!(TimePickerSize::Small.as_class(), "el-time-editor--small");
    }

    #[test]
    fn test_time_value_from_string() {
        let time = TimeValue::from_string("14:30:45").unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);

        let time_short = TimeValue::from_string("09:15").unwrap();
        assert_eq!(time_short.hour, 9);
        assert_eq!(time_short.minute, 15);
        assert_eq!(time_short.second, 0);

        assert!(TimeValue::from_string("").is_none());
        assert!(TimeValue::from_string("invalid").is_none());
    }

    #[test]
    fn test_time_value_to_string() {
        let time = TimeValue::new(14, 5, 9);
        assert_eq!(time.to_string(), "14:05:09");
        assert_eq!(time.to_short_string(), "14:05");
    }

    #[test]
    fn test_is_time_disabled() {
        // No restrictions
        assert!(!is_time_disabled(12, 30, 0, &None, &None));

        // Min time restriction
        assert!(!is_time_disabled(12, 30, 0, &Some("10:00:00".to_string()), &None));
        assert!(is_time_disabled(8, 30, 0, &Some("10:00:00".to_string()), &None));

        // Max time restriction
        assert!(!is_time_disabled(12, 30, 0, &None, &Some("14:00:00".to_string())));
        assert!(is_time_disabled(15, 30, 0, &None, &Some("14:00:00".to_string())));
    }
}
