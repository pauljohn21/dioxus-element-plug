use dioxus::prelude::*;

/// Pagination props - 分页
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Total item count
    #[props(default = 0)]
    pub total: u32,

    /// Items per page
    #[props(default = 10)]
    pub page_size: u32,

    /// Current page number (1-indexed, controlled)
    #[props(default = 1)]
    pub current_page: u32,

    /// Number of page buttons shown
    #[props(default = 7)]
    pub pager_count: u32,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Available page sizes
    #[props(default)]
    pub page_sizes: Vec<u32>,

    /// Whether to show total text
    #[props(default = false)]
    pub show_total: bool,

    /// Current change handler
    #[props(default)]
    pub on_current_change: Option<EventHandler<u32>>,

    /// Size change handler
    #[props(default)]
    pub on_size_change: Option<EventHandler<u32>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Pagination component
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let mut class_names = vec!["el-pagination".to_string()];
    if props.disabled {
        class_names.push("is-disabled".to_string());
    }
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    let total_pages = if props.page_size > 0 { (props.total + props.page_size - 1) / props.page_size } else { 1 };
    let current = props.current_page.min(total_pages).max(1);

    let on_current_change = props.on_current_change;
    let disabled = props.disabled;

    let has_prev = current > 1;
    let has_next = current < total_pages;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            if props.show_total {
                span {
                    class: "el-pagination__total",
                    "共 {props.total} 条"
                }
            }

            button {
                class: "btn-prev",
                disabled: !has_prev || disabled,
                onclick: move |_| {
                    if has_prev && !disabled {
                        if let Some(handler) = on_current_change.as_ref() {
                            handler.call(current - 1);
                        }
                    }
                },
                "‹"
            }

            ul {
                class: "el-pager",

                for page in 1..=total_pages.min(7) {
                    li {
                        class: if page == current { "number active" } else { "number" },
                        onclick: move |_| {
                            if !disabled {
                                if let Some(handler) = on_current_change.as_ref() {
                                    handler.call(page);
                                }
                            }
                        },
                        "{page}"
                    }
                }
            }

            button {
                class: "btn-next",
                disabled: !has_next || disabled,
                onclick: move |_| {
                    if has_next && !disabled {
                        if let Some(handler) = on_current_change.as_ref() {
                            handler.call(current + 1);
                        }
                    }
                },
                "›"
            }
        }
    }
}
