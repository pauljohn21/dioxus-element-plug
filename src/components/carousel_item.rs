use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// CarouselItem props
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    #[props(default)]
    pub children: Element,

    /// Item name/label
    #[props(default)]
    pub name: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// CarouselItem component for individual carousel slides
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let class_string = ClassBuilder::new("el-carousel__item")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
