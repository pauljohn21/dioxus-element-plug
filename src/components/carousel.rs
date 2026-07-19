use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str};

/// Carousel props
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    #[props(default)]
    pub children: Element,

    /// Initial slide index
    #[props(default = 0)]
    pub initial_index: u32,

    /// Slide height
    #[props(default = "300px".to_string())]
    pub height: String,

    /// Trigger type
    #[props(default = "hover".to_string())]
    pub trigger: String,

    /// Whether to autoplay
    #[props(default = true)]
    pub autoplay: bool,

    /// Autoplay interval in ms
    #[props(default = 3000)]
    pub interval: u32,

    /// Indicator position
    #[props(default = "outside".to_string())]
    pub indicator_position: String,

    /// Arrow display
    #[props(default = "hover".to_string())]
    pub arrow: String,

    /// Carousel type
    #[props(default = "default".to_string())]
    pub carousel_type: String,

    #[props(default)]
    pub on_change: Option<EventHandler<u32>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Carousel component for image/content slideshows
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let class_string = ClassBuilder::new("el-carousel")
        .add_if("el-carousel--card", props.carousel_type == "card")
        .add_opt(props.class.as_ref())
        .build();

    rsx! {
        div {
            class: "{class_string}",
            style: "height: {props.height}; {style_str(&props.style)}",
            div {
                class: "el-carousel__container",
                style: "height: {props.height};",
                {props.children}
            }
            div {
                class: "el-carousel__indicators--{props.indicator_position}",
                button {
                    class: "el-carousel__button",
                }
            }
        }
    }
}
