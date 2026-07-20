use super::carousel::CarouselCtx;
use crate::components::common::{style_str, ClassBuilder};
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

/// CarouselItem component for individual carousel slides.
///
/// **Must be used inside a [`Carousel`](super::carousel::Carousel) component.**
/// The active state is determined automatically via context — the item's
/// index is assigned based on its position among siblings.
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let ctx = use_context::<CarouselCtx>();
    let idx = (ctx.counter)();
    {
        let mut c = ctx.counter;
        c.set(idx + 1);
    }
    let is_active = (ctx.active)() == idx;

    let class_string = ClassBuilder::new("el-carousel__item")
        .add_if("is-active", is_active)
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
