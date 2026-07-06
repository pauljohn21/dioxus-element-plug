use dioxus::prelude::*;

/// Progress type
#[derive(Clone, PartialEq)]
pub enum ProgressType {
    Line,
    Circle,
    Dashboard,
}

impl ProgressType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressType::Line => "line",
            ProgressType::Circle => "circle",
            ProgressType::Dashboard => "dashboard",
        }
    }
}

/// Progress status
#[derive(Clone, PartialEq)]
pub enum ProgressStatus {
    Default,
    Success,
    Exception,
    Warning,
}

impl ProgressStatus {
    pub fn as_class(&self) -> &'static str {
        match self {
            ProgressStatus::Default => "",
            ProgressStatus::Success => "is-success",
            ProgressStatus::Exception => "is-exception",
            ProgressStatus::Warning => "is-warning",
        }
    }
}

/// Progress props - 进度条
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Progress type
    #[props(default = ProgressType::Line)]
    pub progress_type: ProgressType,

    /// Percentage (0-100)
    #[props(default = 0)]
    pub percentage: u32,

    /// Current status
    #[props(default = ProgressStatus::Default)]
    pub status: ProgressStatus,

    /// Whether indeterminate
    #[props(default = false)]
    pub indeterminate: bool,

    /// Stroke width in pixels
    #[props(default = 6)]
    pub stroke_width: u32,

    /// Whether to show text
    #[props(default = true)]
    pub show_text: bool,

    /// Custom color
    #[props(default)]
    pub color: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Progress component - progress indicator
///
/// Mirrors Element Plus `el-progress`.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let mut class_names = vec!["el-progress".to_string()];

    class_names.push(format!("el-progress--{}", props.progress_type.as_str()));

    let status_class = props.status.as_class();
    if !status_class.is_empty() {
        class_names.push(status_class.to_string());
    }
    if props.indeterminate {
        class_names.push("is-indeterminate".to_string());
    }
    if !props.show_text {
        class_names.push("el-progress--no-text".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let pct = props.percentage.min(100);
    let bar_color = props.color.as_ref().map(|c| format!("background-color: {};", c)).unwrap_or_default();

    match props.progress_type {
        ProgressType::Line => rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",

                div {
                    class: "el-progress__bar",

                    div {
                        class: "el-progress-bar__outer",
                        style: "height: {props.stroke_width}px;",

                        div {
                            class: "el-progress-bar__inner",
                            style: "width: {pct}%; {bar_color}",
                        }
                    }
                }

                if props.show_text {
                    div {
                        class: "el-progress__text",
                        "{pct}%"
                    }
                }
            }
        },
        ProgressType::Circle | ProgressType::Dashboard => rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",

                div {
                    class: "el-progress__circle",
                    style: "width: 126px; height: 126px;",

                    div {
                        style: "width: 100%; height: 100%; border-radius: 50%; border: {props.stroke_width}px solid #e5e9f2; display: flex; align-items: center; justify-content: center; position: relative;",

                        div {
                            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; border-radius: 50%; border: {props.stroke_width}px solid transparent; border-top-color: #409eff; transform: rotate({pct * 360 / 100}deg);",
                        }

                        span {
                            class: "el-progress__text",
                            style: "position: relative; z-index: 1;",
                            "{pct}%"
                        }
                    }
                }
            }
        },
    }
}
