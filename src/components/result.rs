use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Result icon type
#[derive(Clone, PartialEq)]
pub enum ResultType {
    Success,
    Warning,
    Info,
    Error,
}

impl ResultType {
    pub fn as_class(&self) -> &'static str {
        match self {
            ResultType::Success => "el-result__icon--success",
            ResultType::Warning => "el-result__icon--warning",
            ResultType::Info => "el-result__icon--info",
            ResultType::Error => "el-result__icon--error",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            ResultType::Success => "✓",
            ResultType::Warning => "⚠",
            ResultType::Info => "ℹ",
            ResultType::Error => "✗",
        }
    }
}

/// Result props
#[derive(Props, Clone, PartialEq)]
pub struct ResultProps {
    #[props(default)]
    pub children: Option<Element>,

    #[props(default = ResultType::Info)]
    pub result_type: ResultType,

    #[props(default)]
    pub title: Option<String>,

    #[props(default)]
    pub sub_title: Option<String>,

    #[props(default)]
    pub icon: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Result component for feedback pages
#[component]
pub fn Result(props: ResultProps) -> Element {
    let class_string = ClassBuilder::new("el-result")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            div {
                class: "el-result__icon {props.result_type.as_class()}",
                style: "font-size: 64px;",
                if let Some(ref icon) = props.icon {
                    i { class: "{icon}" }
                } else {
                    "{props.result_type.as_icon()}"
                }
            }
            if let Some(ref title) = props.title {
                div { class: "el-result__title", h2 { "{title}" } }
            }
            if let Some(ref sub) = props.sub_title {
                div { class: "el-result__subtitle", p { "{sub}" } }
            }
            if props.children.is_some() {
                div { class: "el-result__extra", {props.children} }
            }
        }
    }
}
