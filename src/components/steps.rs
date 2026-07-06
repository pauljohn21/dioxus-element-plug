use dioxus::prelude::*;

/// Steps direction
#[derive(Clone, PartialEq)]
pub enum StepsDirection {
    Horizontal,
    Vertical,
}

/// Steps props
#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    #[props(default)]
    pub children: Element,

    /// Current active step
    #[props(default = 0)]
    pub active: u32,

    /// Space between steps
    #[props(default)]
    pub space: Option<String>,

    #[props(default = StepsDirection::Horizontal)]
    pub direction: StepsDirection,

    /// Whether to show the finish status icon
    #[props(default = false)]
    pub simple: bool,

    /// Whether to align center
    #[props(default = false)]
    pub align_center: bool,

    #[props(default)]
    pub on_change: Option<EventHandler<u32>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Steps component for step-by-step processes
#[component]
pub fn Steps(props: StepsProps) -> Element {
    let mut class_names = vec!["el-steps".to_string()];
    match props.direction {
        StepsDirection::Horizontal => class_names.push("is-horizontal".to_string()),
        StepsDirection::Vertical => class_names.push("is-vertical".to_string()),
    }
    if props.simple { class_names.push("is-simple".to_string()); }
    if props.align_center { class_names.push("is-center".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
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

/// Step props
#[derive(Props, Clone, PartialEq)]
pub struct StepProps {
    #[props(default)]
    pub children: Option<Element>,

    /// Step title
    pub title: String,

    /// Step description
    #[props(default)]
    pub description: Option<String>,

    /// Step icon
    #[props(default)]
    pub icon: Option<String>,

    /// Step status
    #[props(default = StepStatus::Wait)]
    pub status: StepStatus,

    #[props(default)]
    pub class: Option<String>,
}

/// Step component for individual steps
#[component]
pub fn Step(props: StepProps) -> Element {
    let mut class_names = vec!["el-step".to_string()];
    class_names.push(props.status.as_class().to_string());
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            div {
                class: "el-step__head",
                div {
                    class: "el-step__icon",
                    if let Some(ref icon) = props.icon {
                        i { class: "{icon}" }
                    } else {
                        span { class: "el-step__icon-inner", "" }
                    }
                }
            }
            div {
                class: "el-step__main",
                div {
                    class: "el-step__title",
                    "{props.title}"
                }
                if let Some(ref desc) = props.description {
                    div {
                        class: "el-step__description",
                        "{desc}"
                    }
                }
            }
        }
    }
}
