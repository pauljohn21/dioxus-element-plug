use crate::components::common::{style_str, ClassBuilder};
use dioxus::prelude::*;

/// TimelineItem timestamp placement
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TimestampPlacement {
    #[default]
    Top,
    Bottom,
}

impl TimestampPlacement {
    pub fn as_class(&self) -> &'static str {
        match self {
            TimestampPlacement::Top => "is-top",
            TimestampPlacement::Bottom => "is-bottom",
        }
    }
}

/// Timeline node type
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TimelineNodeType {
    #[default]
    Normal,
    Large,
    Primary,
}

impl TimelineNodeType {
    pub fn as_class(&self) -> &'static str {
        match self {
            TimelineNodeType::Normal => "",
            TimelineNodeType::Large => "el-timeline-item__node--large",
            TimelineNodeType::Primary => "el-timeline-item__node--primary",
        }
    }
}

/// Timeline props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    /// Timeline children (TimelineItem components)
    #[props(default)]
    pub children: Element,

    /// Reverse chronological order
    #[props(default = false)]
    pub reverse: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Timeline component for displaying chronological events
///
/// Provides a vertical timeline for displaying events in chronological order.
/// Supports custom icons, colors, and node sizes.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::timeline::{Timeline, TimelineItem, TimestampPlacement};
///
/// rsx! {
///     Timeline {
///         TimelineItem {
///             timestamp: Some("2024-01-01".to_string()),
///             placement: TimestampPlacement::Top,
///             "Event content"
///         }
///         TimelineItem {
///             timestamp: Some("2024-01-02".to_string()),
///             color: Some("#ff0000".to_string()),
///             "Another event"
///         }
///     }
/// }
/// ```
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    // Build class string using ClassBuilder
    let class_string = ClassBuilder::new("el-timeline")
        .add_if("is-reverse", props.reverse)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    rsx! {
        ul {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// TimelineItem props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineItemProps {
    /// Timeline item content
    #[props(default)]
    pub children: Option<Element>,

    /// Timestamp text
    #[props(default)]
    pub timestamp: Option<String>,

    /// Timestamp placement (top or bottom)
    #[props(default = TimestampPlacement::Bottom)]
    pub placement: TimestampPlacement,

    /// Custom icon class
    #[props(default)]
    pub icon: Option<String>,

    /// Node color (CSS color value)
    #[props(default)]
    pub color: Option<String>,

    /// Node type
    #[props(default = TimelineNodeType::Normal)]
    pub node_type: TimelineNodeType,

    /// Whether to hide the tail line
    #[props(default = false)]
    pub hide_tail: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// TimelineItem component for individual timeline events
///
/// Represents a single event in the timeline with timestamp,
/// customizable node, and content.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::timeline::{TimelineItem, TimestampPlacement, TimelineNodeType};
///
/// rsx! {
///     TimelineItem {
///         timestamp: Some("2024-01-01".to_string()),
///         placement: TimestampPlacement::Top,
///         color: Some("#409EFF".to_string()),
///         node_type: TimelineNodeType::Primary,
///         icon: Some("el-icon-check".to_string()),
///         "Event description"
///     }
/// }
/// ```
#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    // Build node style with custom color
    let node_style = props
        .color
        .as_ref()
        .map(|c| format!("background-color: {}; border-color: {};", c, c))
        .unwrap_or_default();

    // Build class strings
    let class_string = ClassBuilder::new("el-timeline-item")
        .add_opt(props.class.as_ref())
        .build();

    let node_class = ClassBuilder::new("el-timeline-item__node")
        .add_class(props.node_type.as_class())
        .build();

    rsx! {
        li {
            class: "{class_string}",

            // Tail line (vertical connector)
            if !props.hide_tail {
                div {
                    class: "el-timeline-item__tail",
                }
            }

            // Node (dot/icon container)
            div {
                class: "{node_class}",
                style: "{node_style}",
                if let Some(ref icon) = props.icon {
                    i { class: "{icon}" }
                }
            }

            // Content wrapper
            div {
                class: "el-timeline-item__wrapper",

                // Timestamp (top placement)
                if let Some(ref timestamp) = props.timestamp {
                    if props.placement == TimestampPlacement::Top {
                        div {
                            class: "el-timeline-item__timestamp is-top",
                            "{timestamp}"
                        }
                    }
                }

                // Content
                div {
                    class: "el-timeline-item__content",
                    {props.children}
                }

                // Timestamp (bottom placement)
                if let Some(ref timestamp) = props.timestamp {
                    if props.placement == TimestampPlacement::Bottom {
                        div {
                            class: "el-timeline-item__timestamp is-bottom",
                            "{timestamp}"
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
    fn test_timestamp_placement_as_class() {
        assert_eq!(TimestampPlacement::Top.as_class(), "is-top");
        assert_eq!(TimestampPlacement::Bottom.as_class(), "is-bottom");
    }

    #[test]
    fn test_timeline_node_type_as_class() {
        assert_eq!(TimelineNodeType::Normal.as_class(), "");
        assert_eq!(
            TimelineNodeType::Large.as_class(),
            "el-timeline-item__node--large"
        );
        assert_eq!(
            TimelineNodeType::Primary.as_class(),
            "el-timeline-item__node--primary"
        );
    }

    #[test]
    fn test_timestamp_placement_default() {
        assert_eq!(TimestampPlacement::default(), TimestampPlacement::Top);
    }

    #[test]
    fn test_timeline_node_type_default() {
        assert_eq!(TimelineNodeType::default(), TimelineNodeType::Normal);
    }

    #[test]
    fn test_class_builder_timeline() {
        let class = ClassBuilder::new("el-timeline")
            .add_if("is-reverse", true)
            .build();
        assert_eq!(class, "el-timeline is-reverse");
    }

    #[test]
    fn test_class_builder_timeline_item() {
        let class = ClassBuilder::new("el-timeline-item__node")
            .add_class("el-timeline-item__node--large")
            .build();
        assert_eq!(
            class,
            "el-timeline-item__node el-timeline-item__node--large"
        );
    }
}
