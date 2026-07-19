use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};
use std::fmt;

/// DatePicker type variants
#[derive(Clone, PartialEq, Default)]
pub enum DatePickerType {
    #[default]
    Date,
    Week,
    Month,
    Year,
    Dates,
    Daterange,
    Monthrange,
}

impl DatePickerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatePickerType::Date => "date",
            DatePickerType::Week => "week",
            DatePickerType::Month => "month",
            DatePickerType::Year => "year",
            DatePickerType::Dates => "dates",
            DatePickerType::Daterange => "daterange",
            DatePickerType::Monthrange => "monthrange",
        }
    }

    pub fn placeholder_text(&self) -> &'static str {
        match self {
            DatePickerType::Date => "选择日期",
            DatePickerType::Week => "选择周",
            DatePickerType::Month => "选择月",
            DatePickerType::Year => "选择年",
            DatePickerType::Dates => "选择一个或多个日期",
            DatePickerType::Daterange => "选择日期范围",
            DatePickerType::Monthrange => "选择月份范围",
        }
    }
}

/// DatePicker size variants
#[derive(Clone, PartialEq, Default)]
pub enum DatePickerSize {
    #[default]
    Default,
    Large,
    Small,
}

impl DatePickerSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            DatePickerSize::Default => "",
            DatePickerSize::Large => "el-date-editor--large",
            DatePickerSize::Small => "el-date-editor--small",
        }
    }
}

/// Simple date structure for internal use
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Returns the current date.
    ///
    /// On `wasm32` targets this reads from the browser via `js_sys::Date`;
    /// on native targets it derives the date from `std::time::SystemTime`.
    pub fn today() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let date = js_sys::Date::new_0();
            // JS months are 0-indexed; Rust uses 1-indexed.
            Self::new(
                date.get_full_year() as i32,
                date.get_month() as u32 + 1,
                date.get_date() as u32,
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            let secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let days = (secs / 86400) as i32;
            Self::from_days_since_epoch(days)
        }
    }

    /// Convert days since the Unix epoch (1970-01-01) into a `SimpleDate`.
    ///
    /// Negative inputs produce dates before 1970. The algorithm walks
    /// year-by-year then month-by-month, subtracting days as it goes.
    pub fn from_days_since_epoch(days: i32) -> Self {
        let mut remaining = days;
        let mut year: i32;

        if remaining >= 0 {
            year = 1970;
            loop {
                let year_len = if Self::is_leap_year(year) { 366 } else { 365 };
                if remaining < year_len {
                    break;
                }
                remaining -= year_len;
                year += 1;
            }
        } else {
            year = 1969;
            loop {
                let prev_year_len = if Self::is_leap_year(year) { 366 } else { 365 };
                let needed = remaining + prev_year_len;
                if needed >= 0 {
                    remaining = needed;
                    break;
                }
                remaining = needed;
                year -= 1;
            }
        }

        let mut month: u32 = 1;
        while month <= 12 {
            let month_len = Self::days_in_month(year, month) as i32;
            if remaining < month_len {
                break;
            }
            remaining -= month_len;
            month += 1;
        }

        let day = remaining as u32 + 1;
        Self::new(year, month, day)
    }

    pub fn from_string(date_str: &str) -> Option<Self> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;
        Some(Self::new(year, month, day))
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn weekday(&self) -> u32 {
        // Zeller's congruence algorithm for Gregorian calendar
        let mut year = self.year;
        let mut month = self.month as i32;
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let q = self.day as i32;
        let k = year % 100;
        let j = year / 100;
        let h = (q + 13 * (month + 1) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        // Convert to 0=Sunday, 1=Monday, ...
        (((h + 5) % 7 + 1) % 7) as u32
    }

    pub fn first_day_of_month(&self) -> u32 {
        Self::new(self.year, self.month, 1).weekday()
    }
}

impl fmt::Display for SimpleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// DatePicker props
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Selected date value (format: YYYY-MM-DD)
    #[props(default)]
    pub model_value: Option<String>,

    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,

    /// Picker type
    #[props(default = DatePickerType::Date)]
    pub picker_type: DatePickerType,

    /// Date format
    #[props(default = "YYYY-MM-DD".to_string())]
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
    #[props(default = DatePickerSize::Default)]
    pub size: DatePickerSize,

    /// Minimum selectable date
    #[props(default)]
    pub min_date: Option<String>,

    /// Maximum selectable date
    #[props(default)]
    pub max_date: Option<String>,

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

/// DatePicker component for date selection
///
/// Provides a date input with calendar panel for selecting dates.
/// Supports various picker types: date, week, month, year, daterange.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::date_picker::{DatePicker, DatePickerType};
///
/// rsx! {
///     DatePicker {
///         picker_type: DatePickerType::Date,
///         placeholder: Some("选择日期".to_string()),
///         on_change: move |date: String| println!("Selected: {}", date),
///     }
/// }
/// ```
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let mut panel_visible = use_signal(|| false);
    let mut input_value = use_signal(|| props.model_value.clone().unwrap_or_default());
    let today = SimpleDate::today();
    let mut current_year = use_signal(|| today.year);
    let mut current_month = use_signal(|| today.month);

    // Update input_value when model_value changes
    use_effect(move || {
        if let Some(ref value) = props.model_value {
            input_value.set(value.clone());
            // Parse and set current view
            if let Some(date) = SimpleDate::from_string(value) {
                current_year.set(date.year);
                current_month.set(date.month);
            }
        }
    });

    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| props.picker_type.placeholder_text().to_string());

    // Build class string
    let class_string = ClassBuilder::new("el-date-editor")
        .add_class("el-date-editor--date")
        .add_class(props.size.as_class())
        .add_if("is-disabled", props.disabled)
        .add_if("is-active", panel_visible())
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let on_change = props.on_change;
    let on_focus = props.on_focus;
    let on_blur = props.on_blur;

    // Calendar data
    let year = current_year();
    let month = current_month();
    let days_in_month = SimpleDate::days_in_month(year, month);
    let first_day_weekday = SimpleDate::new(year, month, 1).first_day_of_month();

    // Previous month info
    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let days_in_prev_month = SimpleDate::days_in_month(prev_year, prev_month);

    // Month and weekday names
    let month_names = ["一月", "二月", "三月", "四月", "五月", "六月",
                       "七月", "八月", "九月", "十月", "十一月", "十二月"];
    let week_days = ["日", "一", "二", "三", "四", "五", "六"];

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Input wrapper
            div {
                class: "el-input el-input--default el-range-editor",
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
                    i { class: "el-icon-date" }
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

            // Date picker panel
            if panel_visible() {
                div {
                    class: "el-picker-panel el-date-picker",
                    onclick: move |e: MouseEvent| e.stop_propagation(),

                    // Panel header
                    div {
                        class: "el-picker-panel__header",

                        // Previous year button
                        button {
                            class: "el-picker-panel__icon-btn el-date-picker__prev-year",
                            onclick: move |_| current_year.set(year - 1),
                            "«"
                        }

                        // Previous month button
                        button {
                            class: "el-picker-panel__icon-btn el-date-picker__prev-month",
                            onclick: move |_| {
                                if month == 1 {
                                    current_month.set(12);
                                    current_year.set(year - 1);
                                } else {
                                    current_month.set(month - 1);
                                }
                            },
                            "‹"
                        }

                        // Month/Year display
                        span {
                            class: "el-date-picker__header-label",
                            "{year}年 {month_names[month as usize - 1]}"
                        }

                        // Next month button
                        button {
                            class: "el-picker-panel__icon-btn el-date-picker__next-month",
                            onclick: move |_| {
                                if month == 12 {
                                    current_month.set(1);
                                    current_year.set(year + 1);
                                } else {
                                    current_month.set(month + 1);
                                }
                            },
                            "›"
                        }

                        // Next year button
                        button {
                            class: "el-picker-panel__icon-btn el-date-picker__next-year",
                            onclick: move |_| current_year.set(year + 1),
                            "»"
                        }
                    }

                    // Calendar table
                    table {
                        class: "el-date-table",
                        role: "grid",

                        // Weekday headers
                        thead {
                            tr {
                                for day in week_days {
                                    th { "{day}" }
                                }
                            }
                        }

                        // Calendar days
                        tbody {
                            // Generate 6 weeks
                            for week in 0..6 {
                                tr {
                                    // Generate 7 days per week
                                    for day in 0..7 {
                                        {
                                            let day_index = week * 7 + day;
                                            let (day_num, is_current_month, is_disabled) = if day_index < first_day_weekday as usize {
                                                // Previous month
                                                let day_num = days_in_prev_month - first_day_weekday + day_index as u32 + 1;
                                                let date = SimpleDate::new(prev_year, prev_month, day_num);
                                                let is_disabled = is_date_disabled(&date, &props.min_date, &props.max_date);
                                                (day_num, false, is_disabled)
                                            } else if day_index < first_day_weekday as usize + days_in_month as usize {
                                                // Current month
                                                let day_num = day_index as u32 - first_day_weekday + 1;
                                                let date = SimpleDate::new(year, month, day_num);
                                                let is_disabled = is_date_disabled(&date, &props.min_date, &props.max_date);
                                                (day_num, true, is_disabled)
                                            } else {
                                                // Next month
                                                let next_month_day = day_index as u32 - first_day_weekday - days_in_month + 1;
                                                let next_month = if month == 12 { 1 } else { month + 1 };
                                                let next_year = if month == 12 { year + 1 } else { year };
                                                let date = SimpleDate::new(next_year, next_month, next_month_day);
                                                let is_disabled = is_date_disabled(&date, &props.min_date, &props.max_date);
                                                (next_month_day, false, is_disabled)
                                            };

                                            let date_str = format!("{}-{:02}-{:02}", year, month, day_num);
                                            let is_selected = input_value() == date_str;

                                            let class_name = if is_current_month {
                                                if is_selected {
                                                    "current selected"
                                                } else if is_disabled {
                                                    "disabled"
                                                } else {
                                                    "available"
                                                }
                                            } else {
                                                if is_disabled {
                                                    "disabled"
                                                } else {
                                                    "prev-month"
                                                }
                                            };

                                            rsx! {
                                                td {
                                                    class: "{class_name}",
                                                    onclick: move |_| {
                                                        if !is_disabled && is_current_month {
                                                            let date = format!("{}-{:02}-{:02}", year, month, day_num);
                                                            input_value.set(date.clone());
                                                            fire_event(&on_change, date);
                                                            panel_visible.set(false);
                                                            fire_event(&on_blur, ());
                                                        }
                                                    },
                                                    div {
                                                        span { "{day_num}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Footer with today button
                    div {
                        class: "el-picker-panel__footer",
                        button {
                            class: "el-button el-button--text el-button--mini",
                            onclick: move |_| {
                                let today = SimpleDate::today().to_string();
                                input_value.set(today.clone());
                                fire_event(&on_change, today);
                                panel_visible.set(false);
                                fire_event(&on_blur, ());
                            },
                            "今天"
                        }
                    }
                }
            }
        }
    }
}

/// Check if date is disabled
fn is_date_disabled(date: &SimpleDate, min_date: &Option<String>, max_date: &Option<String>) -> bool {
    if let Some(ref min) = min_date {
        if let Some(min_d) = SimpleDate::from_string(min) {
            if date.year < min_d.year ||
               (date.year == min_d.year && date.month < min_d.month) ||
               (date.year == min_d.year && date.month == min_d.month && date.day < min_d.day) {
                return true;
            }
        }
    }
    if let Some(ref max) = max_date {
        if let Some(max_d) = SimpleDate::from_string(max) {
            if date.year > max_d.year ||
               (date.year == max_d.year && date.month > max_d.month) ||
               (date.year == max_d.year && date.month == max_d.month && date.day > max_d.day) {
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
    fn test_date_picker_type_as_str() {
        assert_eq!(DatePickerType::Date.as_str(), "date");
        assert_eq!(DatePickerType::Week.as_str(), "week");
        assert_eq!(DatePickerType::Month.as_str(), "month");
        assert_eq!(DatePickerType::Year.as_str(), "year");
    }

    #[test]
    fn test_date_picker_size_as_class() {
        assert_eq!(DatePickerSize::Default.as_class(), "");
        assert_eq!(DatePickerSize::Large.as_class(), "el-date-editor--large");
        assert_eq!(DatePickerSize::Small.as_class(), "el-date-editor--small");
    }

    #[test]
    fn test_simple_date_days_in_month() {
        assert_eq!(SimpleDate::days_in_month(2024, 1), 31);  // January
        assert_eq!(SimpleDate::days_in_month(2024, 2), 29);  // February (leap year)
        assert_eq!(SimpleDate::days_in_month(2023, 2), 28);  // February (non-leap year)
        assert_eq!(SimpleDate::days_in_month(2024, 4), 30);  // April
    }

    #[test]
    fn test_simple_date_is_leap_year() {
        assert!(SimpleDate::is_leap_year(2024));
        assert!(SimpleDate::is_leap_year(2000));
        assert!(!SimpleDate::is_leap_year(2023));
        assert!(!SimpleDate::is_leap_year(1900));
    }

    #[test]
    fn test_simple_date_from_string() {
        let date = SimpleDate::from_string("2024-01-15").unwrap();
        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 15);

        assert!(SimpleDate::from_string("").is_none());
        assert!(SimpleDate::from_string("invalid").is_none());
    }

    #[test]
    fn test_simple_date_to_string() {
        let date = SimpleDate::new(2024, 1, 15);
        assert_eq!(date.to_string(), "2024-01-15");
    }

    #[test]
    fn test_simple_date_weekday() {
        // 2024-01-01 was a Monday (1)
        let date = SimpleDate::new(2024, 1, 1);
        assert_eq!(date.weekday(), 1);
    }

    #[test]
    fn test_from_days_since_epoch() {
        // Unix epoch: 1970-01-01 = 0 days
        let epoch = SimpleDate::from_days_since_epoch(0);
        assert_eq!(epoch.year, 1970);
        assert_eq!(epoch.month, 1);
        assert_eq!(epoch.day, 1);

        // 2024-01-01 = 19723 days since epoch
        let d2024 = SimpleDate::from_days_since_epoch(19723);
        assert_eq!(d2024.year, 2024);
        assert_eq!(d2024.month, 1);
        assert_eq!(d2024.day, 1);

        // Cross a month boundary: 1970-02-01 = 31 days
        let feb = SimpleDate::from_days_since_epoch(31);
        assert_eq!(feb.year, 1970);
        assert_eq!(feb.month, 2);
        assert_eq!(feb.day, 1);

        // Leap day: 1972-02-29 = (365 + 365 + 59) = 789 days
        // 1970 (365) + 1971 (365) = 730, + Jan (31) + 28 (Feb to 28th) = 789 → Feb 29
        let leap = SimpleDate::from_days_since_epoch(789);
        assert_eq!(leap.year, 1972);
        assert_eq!(leap.month, 2);
        assert_eq!(leap.day, 29);
    }

    #[test]
    fn test_today_returns_recent_date() {
        let today = SimpleDate::today();
        // Year should be within ±1 of 2026 (allows for clock drift / timezone edge cases).
        assert!(
            (2024..=2028).contains(&today.year),
            "today().year = {} is outside the expected range",
            today.year
        );
        assert!((1..=12).contains(&today.month), "month out of range: {}", today.month);
        assert!((1..=31).contains(&today.day), "day out of range: {}", today.day);
    }

    #[test]
    fn test_is_date_disabled() {
        let date = SimpleDate::new(2024, 6, 15);

        // No restrictions
        assert!(!is_date_disabled(&date, &None, &None));

        // Min date restriction
        assert!(!is_date_disabled(&date, &Some("2024-06-01".to_string()), &None));
        assert!(is_date_disabled(&date, &Some("2024-06-20".to_string()), &None));

        // Max date restriction
        assert!(!is_date_disabled(&date, &None, &Some("2024-06-20".to_string())));
        assert!(is_date_disabled(&date, &None, &Some("2024-06-01".to_string())));
    }
}
