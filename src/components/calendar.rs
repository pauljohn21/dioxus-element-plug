use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str, fire_event};
use crate::components::date_picker::SimpleDate;

/// Calendar cell data for a single day in the 6×7 grid.
///
/// `is_current_month` is `false` for the trailing/leading days that belong
/// to the previous or next month.
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarCell {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub is_current_month: bool,
    pub is_today: bool,
    pub date_str: String,
}

/// Calendar props
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Selected date (YYYY-MM-DD format)
    #[props(default)]
    pub model_value: Option<String>,

    /// First day of week (0 = Sunday, 1 = Monday)
    #[props(default = 1)]
    pub first_day_of_week: u32,

    /// Range restriction
    #[props(default)]
    pub range: Option<(String, String)>,

    #[props(default)]
    pub on_select: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Build the 6×7 (42-cell) calendar grid for the given year/month.
///
/// `first_day_of_week` controls which weekday appears in the first column:
/// `0` → Sunday, `1` → Monday.
pub fn build_calendar_grid(year: i32, month: u32, first_day_of_week: u32) -> Vec<CalendarCell> {
    let today = SimpleDate::today();
    let days_in_current = SimpleDate::days_in_month(year, month);
    let first_weekday = SimpleDate::new(year, month, 1).weekday();

    // Offset of the first day of the current month within the 42-cell grid.
    // Normalize: weekday() returns 0=Sunday..6=Saturday.
    let offset = if first_day_of_week == 0 {
        first_weekday as usize
    } else {
        // Shift so Monday is column 0.
        ((first_weekday as usize + 7) - 1) % 7
    };

    // Previous month info (for trailing days at the start of the grid).
    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12u32)
    } else {
        (year, month - 1)
    };
    let days_in_prev = SimpleDate::days_in_month(prev_year, prev_month);

    // Next month info (for trailing days at the end of the grid).
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1u32)
    } else {
        (year, month + 1)
    };

    let mut cells = Vec::with_capacity(42);
    for i in 0..42usize {
        let (cell_year, cell_month, cell_day, is_current_month) = if i < offset {
            // Previous month trailing days.
            let day = days_in_prev - (offset - i - 1) as u32;
            (prev_year, prev_month, day, false)
        } else if i < offset + days_in_current as usize {
            // Current month days.
            let day = (i - offset) as u32 + 1;
            (year, month, day, true)
        } else {
            // Next month leading days.
            let day = (i - offset - days_in_current as usize) as u32 + 1;
            (next_year, next_month, day, false)
        };

        let is_today = today.year == cell_year
            && today.month == cell_month
            && today.day == cell_day;

        cells.push(CalendarCell {
            year: cell_year,
            month: cell_month,
            day: cell_day,
            is_current_month,
            is_today,
            date_str: format!("{:04}-{:02}-{:02}", cell_year, cell_month, cell_day),
        });
    }

    cells
}

/// Calendar component for date selection
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let today = SimpleDate::today();
    let mut current_year = use_signal(|| today.year);
    let mut current_month = use_signal(|| today.month);

    let class_string = ClassBuilder::new("el-calendar")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let year = current_year();
    let month = current_month();
    let first_day_of_week = props.first_day_of_week;

    // Weekday headers depend on first_day_of_week.
    let weekday_labels: Vec<&str> = if first_day_of_week == 0 {
        vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
    } else {
        vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    };

    // Pre-compute the 42-cell grid.
    let cells = build_calendar_grid(year, month, first_day_of_week);
    let selected = props.model_value.clone();
    let on_select = props.on_select;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            div {
                class: "el-calendar__header",

                div {
                    class: "el-calendar__title",
                    "{year} 年 {month} 月"
                }

                div {
                    class: "el-calendar__button-group",

                    button {
                        class: "el-button el-button--plain el-button--mini",
                        r#type: "button",
                        onclick: move |_| {
                            if month == 1 {
                                current_month.set(12);
                                current_year.set(year - 1);
                            } else {
                                current_month.set(month - 1);
                            }
                        },
                        "上个月"
                    }

                    button {
                        class: "el-button el-button--plain el-button--mini",
                        r#type: "button",
                        onclick: move |_| {
                            current_year.set(today.year);
                            current_month.set(today.month);
                        },
                        "今天"
                    }

                    button {
                        class: "el-button el-button--plain el-button--mini",
                        r#type: "button",
                        onclick: move |_| {
                            if month == 12 {
                                current_month.set(1);
                                current_year.set(year + 1);
                            } else {
                                current_month.set(month + 1);
                            }
                        },
                        "下个月"
                    }
                }
            }

            div {
                class: "el-calendar__body",

                table {
                    class: "el-calendar-table",

                    thead {
                        tr {
                            for label in weekday_labels.iter() {
                                th { class: "el-calendar-table__head", "{label}" }
                            }
                        }
                    }

                    tbody {
                        for week in 0..6usize {
                            tr {
                                for day_in_week in 0..7usize {
                                    {
                                        let idx = week * 7 + day_in_week;
                                        let cell = &cells[idx];
                                        let is_selected = selected.as_ref() == Some(&cell.date_str);
                                        let cell_class = ClassBuilder::new("el-calendar-day")
                                            .add_if("is-selected", is_selected)
                                            .add_if("is-today", cell.is_today)
                                            .add_if("is-not-current-month", !cell.is_current_month)
                                            .build();
                                        let date_str = cell.date_str.clone();
                                        let is_current_month = cell.is_current_month;
                                        let day_num = cell.day;

                                        rsx! {
                                            td {
                                                class: "{cell_class}",
                                                onclick: move |_| {
                                                    if is_current_month {
                                                        fire_event(&on_select, date_str.clone());
                                                    }
                                                },
                                                "{day_num}"
                                            }
                                        }
                                    }
                                }
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
    fn test_calendar_grid_has_42_cells() {
        let grid = build_calendar_grid(2024, 1, 1);
        assert_eq!(grid.len(), 42);
    }

    #[test]
    fn test_calendar_grid_current_month_days() {
        // January 2024 has 31 days.
        let grid = build_calendar_grid(2024, 1, 1);
        let current_month_days: Vec<&CalendarCell> =
            grid.iter().filter(|c| c.is_current_month).collect();
        assert_eq!(current_month_days.len(), 31);

        // First current-month day should be day 1.
        assert_eq!(current_month_days[0].day, 1);
        // Last current-month day should be day 31.
        assert_eq!(current_month_days.last().unwrap().day, 31);
    }

    #[test]
    fn test_calendar_grid_february_leap_year() {
        // February 2024 has 29 days (leap year).
        let grid = build_calendar_grid(2024, 2, 1);
        let count = grid.iter().filter(|c| c.is_current_month).count();
        assert_eq!(count, 29);
    }

    #[test]
    fn test_calendar_grid_february_non_leap_year() {
        // February 2023 has 28 days.
        let grid = build_calendar_grid(2023, 2, 1);
        let count = grid.iter().filter(|c| c.is_current_month).count();
        assert_eq!(count, 28);
    }

    #[test]
    fn test_calendar_grid_first_day_of_week_monday() {
        // 2024-01-01 is a Monday. With first_day_of_week=1 (Monday),
        // day 1 should appear in column 0 of the first week (index 0).
        let grid = build_calendar_grid(2024, 1, 1);
        let first_cell = &grid[0];
        assert!(first_cell.is_current_month);
        assert_eq!(first_cell.day, 1);
    }

    #[test]
    fn test_calendar_grid_first_day_of_week_sunday() {
        // 2024-01-01 is a Monday. With first_day_of_week=0 (Sunday),
        // day 1 should appear in column 1 of the first week (index 1).
        let grid = build_calendar_grid(2024, 1, 0);
        // Index 0 should be a trailing December day.
        assert!(!grid[0].is_current_month);
        assert_eq!(grid[0].month, 12);
        // Index 1 should be January 1.
        assert!(grid[1].is_current_month);
        assert_eq!(grid[1].day, 1);
    }

    #[test]
    fn test_calendar_grid_date_strings() {
        let grid = build_calendar_grid(2024, 1, 1);
        let first_current = grid.iter().find(|c| c.is_current_month).unwrap();
        assert_eq!(first_current.date_str, "2024-01-01");

        let last_current = grid.iter().rev().find(|c| c.is_current_month).unwrap();
        assert_eq!(last_current.date_str, "2024-01-31");
    }

    #[test]
    fn test_calendar_grid_today_flag() {
        let today = SimpleDate::today();
        let grid = build_calendar_grid(today.year, today.month, 1);
        let today_cells: Vec<&CalendarCell> = grid.iter().filter(|c| c.is_today).collect();
        assert_eq!(today_cells.len(), 1, "exactly one cell should be flagged as today");
        assert_eq!(today_cells[0].day, today.day);
        assert!(today_cells[0].is_current_month);
    }
}
