use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Steps direction
#[derive(Clone, PartialEq, Default, Debug)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl StepsDirection {
    pub fn as_class(&self) -> &'static str {
        match self {
            StepsDirection::Horizontal => "is-horizontal",
            StepsDirection::Vertical => "is-vertical",
        }
    }
}

/// Steps props
#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    /// Step children
    #[props(default)]
    pub children: Element,

    /// Current active step (0-based index)
    #[props(default = 0)]
    pub active: u32,

    /// Space between steps
    #[props(default)]
    pub space: Option<String>,

    /// Direction: Horizontal or Vertical
    #[props(default = StepsDirection::Horizontal)]
    pub direction: StepsDirection,

    /// Simple mode (no descriptions)
    #[props(default = false)]
    pub simple: bool,

    /// Center align steps
    #[props(default = false)]
    pub align_center: bool,

    /// Step change callback
    #[props(default)]
    pub on_change: Option<EventHandler<u32>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Steps component for step-by-step processes
///
/// Provides a visual indicator for multi-step workflows with
/// support for horizontal/vertical layouts and various states.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::steps::{Steps, Step, StepsDirection};
///
/// rsx! {
///     Steps {
///         active: 1,
///         direction: StepsDirection::Horizontal,
///         Step { title: "Step 1", description: Some("Description 1".to_string()) }
///         Step { title: "Step 2", description: Some("Description 2".to_string()) }
///         Step { title: "Step 3" }
///     }
/// }
/// ```
#[component]
pub fn Steps(props: StepsProps) -> Element {
    // Build class string using ClassBuilder
    let class_string = ClassBuilder::new("el-steps")
        .add_class(props.direction.as_class())
        .add_if("is-simple", props.simple)
        .add_if("is-center", props.align_center)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let _on_change = props.on_change;

    // Calculate style for space between steps
    let steps_style = if let Some(ref space) = props.space {
        format!("{} --el-steps-space: {};", style_string, space)
    } else {
        style_string
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{steps_style}",
            {props.children}
        }
    }
}

/// Step status
#[derive(Clone, PartialEq, Debug)]
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

    /// Calculate status based on step index and active index
    pub fn from_index(step_index: u32, active_index: u32) -> Self {
        if step_index < active_index {
            StepStatus::Finish
        } else if step_index == active_index {
            StepStatus::Process
        } else {
            StepStatus::Wait
        }
    }
}

/// Step props
#[derive(Props, Clone, PartialEq)]
pub struct StepProps {
    /// Step content (optional)
    #[props(default)]
    pub children: Option<Element>,

    /// Step title (required)
    pub title: String,

    /// Step description
    #[props(default)]
    pub description: Option<String>,

    /// Custom icon class
    #[props(default)]
    pub icon: Option<String>,

    /// Step status (auto-calculated if None)
    #[props(default)]
    pub status: Option<StepStatus>,

    /// Step index (set by parent Steps component)
    #[props(default)]
    pub index: Option<u32>,

    /// Total number of steps (set by parent Steps component)
    #[props(default)]
    pub total: Option<u32>,

    /// Active step index (set by parent Steps component)
    #[props(default)]
    pub active_index: Option<u32>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Step component for individual steps
///
/// Represents a single step within a Steps component.
/// Status is automatically calculated based on the active step index
/// unless explicitly overridden.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::steps::Step;
///
/// rsx! {
///     Step {
///         title: "Step 1",
///         description: Some("Description".to_string()),
///         icon: Some("el-icon-edit".to_string()),
///     }
/// }
/// ```
#[component]
pub fn Step(props: StepProps) -> Element {
    // Calculate status based on index and active_index
    let status = if let Some(ref status) = props.status {
        status.clone()
    } else if let (Some(index), Some(active)) = (props.index, props.active_index) {
        StepStatus::from_index(index, active)
    } else {
        StepStatus::Wait
    };

    // Determine if this is the last step
    let is_last = match (props.index, props.total) {
        (Some(idx), Some(total)) => idx == total - 1,
        _ => false,
    };

    // Build class string
    let class_string = ClassBuilder::new("el-step")
        .add_class(status.as_class())
        .add_if("is-last", is_last)
        .add_opt(props.class.as_ref())
        .build();

    // Determine icon content
    let icon_content = if let Some(ref icon) = props.icon {
        rsx! { i { class: "{icon}" } }
    } else {
        // Use step number or status icon
        match status {
            StepStatus::Finish | StepStatus::Success => {
                rsx! { i { class: "el-icon-check" } }
            }
            StepStatus::Error => {
                rsx! { i { class: "el-icon-close" } }
            }
            _ => {
                // Show step number (1-based)
                let step_number = props.index.map(|i| i + 1).unwrap_or(0);
                rsx! { span { class: "el-step__icon-inner", "{step_number}" } }
            }
        }
    };

    rsx! {
        div {
            class: "{class_string}",

            // Step head with icon and line
            div {
                class: "el-step__head",

                // Line connector (not on last step)
                if !is_last {
                    div {
                        class: "el-step__line",
                    }
                }

                // Icon container
                div {
                    class: "el-step__icon",
                    {icon_content}
                }
            }

            // Step main content
            div {
                class: "el-step__main",

                // Title
                div {
                    class: "el-step__title",
                    "{props.title}"
                }

                // Description (if provided)
                if let Some(ref desc) = props.description {
                    div {
                        class: "el-step__description",
                        "{desc}"
                    }
                }

                // Children content (if provided)
                if let Some(children) = props.children {
                    div {
                        class: "el-step__content",
                        {children}
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
    fn test_steps_direction_as_class() {
        assert_eq!(StepsDirection::Horizontal.as_class(), "is-horizontal");
        assert_eq!(StepsDirection::Vertical.as_class(), "is-vertical");
    }

    #[test]
    fn test_step_status_as_class() {
        assert_eq!(StepStatus::Wait.as_class(), "is-wait");
        assert_eq!(StepStatus::Process.as_class(), "is-process");
        assert_eq!(StepStatus::Finish.as_class(), "is-finish");
        assert_eq!(StepStatus::Error.as_class(), "is-error");
        assert_eq!(StepStatus::Success.as_class(), "is-success");
    }

    #[test]
    fn test_step_status_from_index() {
        // active = 2, step indices: 0, 1, 2, 3
        assert_eq!(StepStatus::from_index(0, 2), StepStatus::Finish); // before active
        assert_eq!(StepStatus::from_index(1, 2), StepStatus::Finish); // before active
        assert_eq!(StepStatus::from_index(2, 2), StepStatus::Process); // active
        assert_eq!(StepStatus::from_index(3, 2), StepStatus::Wait); // after active
    }

    #[test]
    fn test_steps_direction_default() {
        assert_eq!(StepsDirection::default(), StepsDirection::Horizontal);
    }

    #[test]
    fn test_class_builder_steps() {
        let class = ClassBuilder::new("el-steps")
            .add_class("is-horizontal")
            .add_if("is-simple", true)
            .add_if("is-center", false)
            .build();
        assert_eq!(class, "el-steps is-horizontal is-simple");
    }

    #[test]
    fn test_class_builder_step() {
        let class = ClassBuilder::new("el-step")
            .add_class("is-process")
            .add_if("is-last", true)
            .build();
        assert_eq!(class, "el-step is-process is-last");
    }
}
