use dioxus::prelude::*;
use crate::components::common::{ClassBuilder, style_str, fire_event};

/// Pagination props
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Total number of items
    #[props(default = 0)]
    pub total: u32,

    /// Current page number
    #[props(default = 1)]
    pub current_page: u32,

    /// Items per page
    #[props(default = 10)]
    pub page_size: u32,

    /// Number of page buttons to show
    #[props(default = 7)]
    pub pager_count: u32,

    /// Whether to show the total count
    #[props(default = false)]
    pub show_total: bool,

    /// Whether to show the page size selector
    #[props(default = false)]
    pub show_size_picker: bool,

    /// Whether to show the "Go to" input
    #[props(default = false)]
    pub show_jumper: bool,

    /// Layout sections
    #[props(default = "prev, pager, next".to_string())]
    pub layout: String,

    /// Whether the pagination is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to use small size
    #[props(default = false)]
    pub small: bool,

    /// Page size options
    #[props(default)]
    pub page_sizes: Vec<u32>,

    /// Change event handler (current page)
    #[props(default)]
    pub on_current_change: Option<EventHandler<u32>>,

    /// Change event handler (page size)
    #[props(default)]
    pub on_size_change: Option<EventHandler<u32>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Pagination component for navigating paged data
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let class_string = ClassBuilder::new("el-pagination")
        .add_if("el-pagination--small", props.small)
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_current_change = props.on_current_change;

    let total_pages = if props.page_size > 0 {
        props.total.div_ceil(props.page_size)
    } else {
        1
    };
    let total_pages = total_pages.max(1);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            role: "pagination",

            if props.show_total {
                span {
                    class: "el-pagination__total",
                    "Total {props.total}"
                }
            }

            // Previous button
            button {
                class: "btn-prev el-pagination__button",
                disabled: props.disabled || props.current_page <= 1,
                onclick: move |_| {
                    if !props.disabled && props.current_page > 1 {
                        fire_event(&on_current_change, props.current_page - 1);
                    }
                },
                "‹"
            }

            // Page numbers
            span {
                class: "el-pager",
                for page in 1..=total_pages.min(props.pager_count) {
                    button {
                        class: {
                            let mut cls = "el-pager__number".to_string();
                            if page == props.current_page { cls.push_str(" is-active"); }
                            cls
                        },
                        onclick: move |_| {
                            if !props.disabled {
                                fire_event(&on_current_change, page);
                            }
                        },
                        "{page}"
                    }
                }
                if total_pages > props.pager_count {
                    span { class: "el-icon-more", "..." }
                }
            }

            // Next button
            button {
                class: "btn-next el-pagination__button",
                disabled: props.disabled || props.current_page >= total_pages,
                onclick: move |_| {
                    if !props.disabled && props.current_page < total_pages {
                        fire_event(&on_current_change, props.current_page + 1);
                    }
                },
                "›"
            }

            if props.show_jumper {
                span {
                    class: "el-pagination__jump",
                    "Go to"
                    input {
                        r#type: "number",
                        class: "el-pagination__editor",
                        value: "{props.current_page}",
                        disabled: props.disabled,
                    }
                    "pages"
                }
            }
        }
    }
}
