use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Context for passing active index to CarouselItem via context provider.
#[derive(Clone, Copy)]
pub struct CarouselCtx {
    /// Active slide index
    pub active: Signal<u32>,
    /// Counter for CarouselItem index assignment (reset each render)
    pub counter: Signal<u32>,
}

/// Cross-platform async sleep function for autoplay timer.
async fn sleep_ms(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms.into())).await;
    }
}

/// Carousel props
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    #[props(default)]
    pub children: Element,

    /// Controlled active slide index
    #[props(default)]
    pub model_value: Option<u32>,

    /// Initial slide index (used when model_value is None)
    #[props(default = 0)]
    pub initial_index: u32,

    /// Slide height
    #[props(default = "300px".to_string())]
    pub height: String,

    /// Whether to autoplay
    #[props(default = true)]
    pub autoplay: bool,

    /// Autoplay interval in ms
    #[props(default = 3000)]
    pub interval: u32,

    /// Indicator position: "outside" or "none"
    #[props(default = "outside".to_string())]
    pub indicator_position: String,

    /// Arrow display: "always", "hover", "never"
    #[props(default = "hover".to_string())]
    pub arrow: String,

    /// Carousel type: "default" or "card"
    #[props(default = "default".to_string())]
    pub carousel_type: String,

    /// Number of slides (for indicator rendering)
    #[props(default = 0)]
    pub length: u32,

    /// Trigger type: "hover" or "click"
    #[props(default = "hover".to_string())]
    pub trigger: String,

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
    let current = props.model_value.unwrap_or(props.initial_index);
    let ctx = use_context_provider(|| CarouselCtx {
        active: Signal::new(current),
        counter: Signal::new(0),
    });

    // Update active index and reset counter on each render
    {
        let mut a = ctx.active;
        a.set(current);
        let mut c = ctx.counter;
        c.set(0);
    }

    let class_string = ClassBuilder::new("el-carousel")
        .add_if("el-carousel--card", props.carousel_type == "card")
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);
    let on_change = props.on_change;
    let length = props.length;
    let interval = props.interval;
    let autoplay = props.autoplay;
    let active = ctx.active;

    // Autoplay timer — automatically advances slides
    use_resource(move || async move {
        if !autoplay || length == 0 {
            return;
        }
            loop {
            sleep_ms(interval).await;
            let next = next_index((active)(), length);
            let mut a = active;
            a.set(next);
            fire_event(&on_change, next);
        }
    });

    rsx! {
        div {
            class: "{class_string}",
            style: "height: {props.height}; {style_string}",
            div {
                class: "el-carousel__container",
                style: "height: {props.height}; position: relative; overflow: hidden;",
                {props.children}

                // Prev/next arrows
                if props.arrow != "never" {
                    button {
                        class: "el-carousel__arrow el-carousel__arrow--left",
                        style: "position: absolute; top: 50%; left: 16px; transform: translateY(-50%); z-index: 10; \
                            background: rgba(31,45,61,0.11); border: none; border-radius: 50%; width: 36px; height: 36px; \
                            color: #fff; cursor: pointer; font-size: 16px;",
                        onclick: move |_| {
                            let next = prev_index((active)(), length);
                            let mut a = active;
                            a.set(next);
                            fire_event(&on_change, next);
                        },
                        "‹"
                    }
                    button {
                        class: "el-carousel__arrow el-carousel__arrow--right",
                        style: "position: absolute; top: 50%; right: 16px; transform: translateY(-50%); z-index: 10; \
                            background: rgba(31,45,61,0.11); border: none; border-radius: 50%; width: 36px; height: 36px; \
                            color: #fff; cursor: pointer; font-size: 16px;",
                        onclick: move |_| {
                            let next = next_index((active)(), length);
                            let mut a = active;
                            a.set(next);
                            fire_event(&on_change, next);
                        },
                        "›"
                    }
                }
            }

            // Indicators
            if length > 0 && props.indicator_position != "none" {
                div {
                    class: "el-carousel__indicators el-carousel__indicators--{props.indicator_position}",
                    style: "display: flex; justify-content: center; gap: 4px; margin-top: 10px;",
                    for i in 0..length {
                        button {
                            class: "el-carousel__indicator",
                            class: if (active)() == i { "is-active" },
                            style: "background: #c0ccda; border: none; border-radius: 2px; width: 24px; height: 3px; cursor: pointer; padding: 0;",
                            onclick: move |_| {
                                let mut a = active;
                                a.set(i);
                                fire_event(&on_change, i);
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Compute the next slide index with wrap-around.
/// Pure function for testing.
fn next_index(current: u32, length: u32) -> u32 {
    if length == 0 {
        return current;
    }
    (current + 1) % length
}

/// Compute the previous slide index with wrap-around.
/// Pure function for testing.
fn prev_index(current: u32, length: u32) -> u32 {
    if length == 0 {
        return current;
    }
    if current == 0 {
        length - 1
    } else {
        current - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_index_normal() {
        assert_eq!(next_index(0, 3), 1);
        assert_eq!(next_index(1, 3), 2);
    }

    #[test]
    fn test_next_index_wrap_around() {
        assert_eq!(next_index(2, 3), 0);
    }

    #[test]
    fn test_next_index_zero_length() {
        assert_eq!(next_index(0, 0), 0);
    }

    #[test]
    fn test_prev_index_normal() {
        assert_eq!(prev_index(2, 3), 1);
        assert_eq!(prev_index(1, 3), 0);
    }

    #[test]
    fn test_prev_index_wrap_around() {
        assert_eq!(prev_index(0, 3), 2);
    }

    #[test]
    fn test_prev_index_zero_length() {
        assert_eq!(prev_index(0, 0), 0);
    }
}
