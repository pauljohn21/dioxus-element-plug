use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

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

/// Progress props
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

    /// Whether to set indeterminate progress
    #[props(default = false)]
    pub indeterminate: bool,

    /// Stroke width in pixels
    #[props(default = 6)]
    pub stroke_width: u32,

    /// Whether to place percentage inside progress bar
    #[props(default = false)]
    pub text_inside: bool,

    /// Canvas width for circle/dashboard type
    #[props(default = 126)]
    pub width: u32,

    /// Whether to show percentage text
    #[props(default = true)]
    pub show_text: bool,

    /// Background color of progress bar
    #[props(default)]
    pub color: Option<String>,

    /// Whether to show stripes
    #[props(default = false)]
    pub striped: bool,

    /// Whether stripes flow
    #[props(default = false)]
    pub striped_flow: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Progress component for showing task completion
///
/// ## Example
///
/// ```rust,ignore
/// rsx! {
///     Progress { percentage: 70, status: ProgressStatus::Success }
/// }
/// ```
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let percentage = props.percentage.min(100);

    let type_class = format!("el-progress--{}", props.progress_type.as_str());
    let class_string = ClassBuilder::new("el-progress")
        .add_class(&type_class)
        .add_class(props.status.as_class())
        .add_if("el-progress--indeterminate", props.indeterminate)
        .add_if("el-progress--striped", props.striped)
        .add_if("el-progress--striped-flow", props.striped_flow)
        .add_if("el-progress--text-inside", props.text_inside && props.progress_type == ProgressType::Line)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let bar_color = props.color.clone().unwrap_or_else(|| {
        match props.status {
            ProgressStatus::Success => "#67C23A".to_string(),
            ProgressStatus::Exception => "#F56C6C".to_string(),
            ProgressStatus::Warning => "#E6A23C".to_string(),
            ProgressStatus::Default => "#409EFF".to_string(),
        }
    });

    let display_text = format!("{}%", percentage);

    if props.progress_type == ProgressType::Line {
        let bar_style = format!("width: {}%; background-color: {};", percentage, bar_color);
        let height_style = format!("height: {}px;", props.stroke_width);

        rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
                div {
                    class: "el-progress__outer",
                    style: "{height_style}",
                    div {
                        class: "el-progress__inner",
                        div {
                            class: "el-progress__bar",
                            style: "{bar_style}",
                            if props.text_inside && props.show_text {
                                div {
                                    class: "el-progress__text el-progress__text--inside",
                                    "{display_text}"
                                }
                            }
                        }
                    }
                }
                if props.show_text && !props.text_inside {
                    div {
                        class: "el-progress__text",
                        if props.status == ProgressStatus::Success {
                            i { class: "el-icon-circle-check" }
                        } else if props.status == ProgressStatus::Exception {
                            i { class: "el-icon-circle-close" }
                        } else if props.status == ProgressStatus::Warning {
                            i { class: "el-icon-warning" }
                        } else {
                            "{display_text}"
                        }
                    }
                }
            }
        }
    } else {
        // Circle / Dashboard - using CSS conic-gradient
        let size = props.width;
        let deg = (percentage as f64 / 100.0) * 360.0;
        let bg_gradient = if props.progress_type == ProgressType::Dashboard {
            format!("conic-gradient(from 135deg, {} 0deg, {} {}deg, transparent {}deg)",
                bar_color, bar_color, deg * 0.75, deg * 0.75)
        } else {
            format!("conic-gradient({} {}deg, var(--el-fill-color-light) {}deg)",
                bar_color, deg, deg)
        };

        let inner_size = size - props.stroke_width * 2;

        rsx! {
            div {
                class: "{class_string}",
                style: "{style_string}",
                div {
                    class: "el-progress-circle",
                    style: "width: {size}px; height: {size}px; border-radius: 50%; background: {bg_gradient}; display: flex; align-items: center; justify-content: center;",
                    div {
                        style: "width: {inner_size}px; height: {inner_size}px; border-radius: 50%; background: var(--el-bg-color); display: flex; align-items: center; justify-content: center;",
                        if props.show_text {
                            div {
                                class: "el-progress__text",
                                if props.status == ProgressStatus::Success {
                                    i { class: "el-icon-circle-check" }
                                } else if props.status == ProgressStatus::Exception {
                                    i { class: "el-icon-circle-close" }
                                } else {
                                    "{display_text}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
