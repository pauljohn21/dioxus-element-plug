use dioxus::prelude::*;

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
    let mut class_names = vec!["el-carousel__item".to_string()];
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            {props.children}
        }
    }
}
