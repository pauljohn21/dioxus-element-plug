use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Space direction
#[derive(Clone, PartialEq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

/// Space alignment
#[derive(Clone, PartialEq)]
pub enum SpaceAlignment {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl SpaceAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpaceAlignment::Start => "start",
            SpaceAlignment::Center => "center",
            SpaceAlignment::End => "end",
            SpaceAlignment::Baseline => "baseline",
            SpaceAlignment::Stretch => "stretch",
        }
    }
}

/// Space props
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    #[props(default)]
    pub children: Element,

    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,

    #[props(default = SpaceAlignment::Center)]
    pub alignment: SpaceAlignment,

    #[props(default = "8px".to_string())]
    pub size: String,

    #[props(default = false)]
    pub wrap: bool,

    #[props(default = false)]
    pub fill: bool,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Space component for consistent spacing between elements
#[component]
pub fn Space(props: SpaceProps) -> Element {
    let class_string = ClassBuilder::new("el-space")
        .add_if("el-space--fill", props.fill)
        .add_opt(props.class.as_ref())
        .build();

    let dir = match props.direction {
        SpaceDirection::Horizontal => "row",
        SpaceDirection::Vertical => "column",
    };
    let wrap = if props.wrap { "wrap" } else { "nowrap" };

    rsx! {
        div {
            class: "{class_string}",
            style: "display: flex; flex-direction: {dir}; flex-wrap: {wrap}; align-items: {props.alignment.as_str()}; gap: {props.size}; {style_str(&props.style)}",
            {props.children}
        }
    }
}
