use dioxus::prelude::*;

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

/// Calendar component for date selection
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let mut class_names = vec!["el-calendar".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let _days: Vec<u32> = (1..=31).collect();

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            div {
                class: "el-calendar__header",
                div {
                    class: "el-calendar__title",
                    "Calendar"
                }
            }
            div {
                class: "el-calendar__body",
                table {
                    class: "el-calendar-table",
                    thead {
                        tr {
                            for day in weekdays.iter() {
                                th { class: "el-calendar-table__head", "{day}" }
                            }
                        }
                    }
                    tbody {
                        for week in 0..6 {
                            tr {
                                for _day in 0..7 {
                                    td {
                                        class: "el-calendar-day",
                                        "{week * 7 + 1}"
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
