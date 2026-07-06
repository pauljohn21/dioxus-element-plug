use dioxus::prelude::*;

/// TimelineItem timestamp placement
#[derive(Clone, PartialEq)]
pub enum TimestampPlacement {
    Top,
    Bottom,
}

/// Timeline props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    #[props(default)]
    pub children: Element,

    #[props(default = false)]
    pub reverse: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Timeline component for displaying chronological events
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let mut class_names = vec!["el-timeline".to_string()];
    if props.reverse { class_names.push("is-reverse".to_string()); }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        ul {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
}

/// TimelineItem props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineItemProps {
    #[props(default)]
    pub children: Option<Element>,

    #[props(default)]
    pub timestamp: Option<String>,

    #[props(default = TimestampPlacement::Bottom)]
    pub placement: TimestampPlacement,

    #[props(default)]
    pub icon: Option<String>,

    #[props(default)]
    pub color: Option<String>,

    #[props(default = "default".to_string())]
    pub node_type: String,

    #[props(default)]
    pub class: Option<String>,
}

/// TimelineItem component for individual timeline events
#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    let dot_style = props.color.clone().map(|c| format!("background-color: {};", c)).unwrap_or_default();

    rsx! {
        li {
            class: "el-timeline-item",
            div {
                class: "el-timeline-item__tail",
            }
            div {
                class: "el-timeline-item__node el-timeline-item__node--{props.node_type}",
                style: "{dot_style}",
                if let Some(ref icon) = props.icon {
                    i { class: "{icon}" }
                }
            }
            div {
                class: "el-timeline-item__wrapper",
                if let Some(ref timestamp) = props.timestamp {
                    if props.placement == TimestampPlacement::Top {
                        div { class: "el-timeline-item__timestamp is-top", "{timestamp}" }
                    }
                }
                div {
                    class: "el-timeline-item__content",
                    {props.children}
                }
                if let Some(ref timestamp) = props.timestamp {
                    if props.placement == TimestampPlacement::Bottom {
                        div { class: "el-timeline-item__timestamp is-bottom", "{timestamp}" }
                    }
                }
            }
        }
    }
}
