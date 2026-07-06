use dioxus::prelude::*;

/// Steps direction
#[derive(Clone, PartialEq)]
pub enum StepsDirection {
    Horizontal,
    Vertical,
}

/// Steps props - 步骤条
#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    /// Steps content (Step components)
    pub children: Element,

    /// Current active step (0-indexed)
    #[props(default = 0)]
    pub active: u32,

    /// Direction
    #[props(default = StepsDirection::Horizontal)]
    pub direction: StepsDirection,

    /// Process status
    #[props(default = StepStatus::Process)]
    pub process_status: StepStatus,

    /// Finish status
    #[props(default = StepStatus::Finish)]
    pub finish_status: StepStatus,

    /// Space between steps (CSS value)
    #[props(default)]
    pub space: Option<String>,

    /// Whether to align center
    #[props(default = false)]
    pub align_center: bool,

    /// Whether simple style
    #[props(default = false)]
    pub simple: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Step status
#[derive(Clone, PartialEq)]
pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
    Success,
}

impl StepStatus {
    pub fn as_class(&self) -> &'static str {
        match self {
            StepStatus::Wait => "is-wait",
            StepStatus::Process => "is-process",
            StepStatus::Finish => "is-finish",
            StepStatus::Error => "is-error",
            StepStatus::Success => "is-success",
        }
    }
}

/// Steps component
#[component]
pub fn Steps(props: StepsProps) -> Element {
    let mut class_names = vec!["el-steps".to_string()];

    match props.direction {
        StepsDirection::Horizontal => {}
        StepsDirection::Vertical => class_names.push("el-steps--vertical".to_string()),
    }

    if props.simple {
        class_names.push("el-steps--simple".to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Step props
#[derive(Props, Clone, PartialEq)]
pub struct StepProps {
    /// Step content
    pub children: Element,

    /// Step title
    #[props(default)]
    pub title: Option<String>,

    /// Step description
    #[props(default)]
    pub description: Option<String>,

    /// Step icon
    #[props(default)]
    pub icon: Option<String>,

    /// Step status
    #[props(default = StepStatus::Wait)]
    pub status: StepStatus,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Step component
#[component]
pub fn Step(props: StepProps) -> Element {
    let mut class_names = vec!["el-step".to_string()];
    class_names.push(props.status.as_class().to_string());
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    rsx! {
        div {
            class: "{class_string}",

            div {
                class: "el-step__head",

                div {
                    class: "el-step__line",
                }

                div {
                    class: "el-step__icon",
                    if let Some(ref icon) = props.icon {
                        i { class: "{icon}" }
                    } else {
                        span { class: "el-step__icon-inner", "●" }
                    }
                }
            }

            div {
                class: "el-step__main",

                div {
                    class: "el-step__title",
                    if let Some(ref t) = props.title {
                        "{t}"
                    }
                }

                div {
                    class: "el-step__description",
                    if let Some(ref d) = props.description {
                        "{d}"
                    }
                }
            }
        }
    }
}
