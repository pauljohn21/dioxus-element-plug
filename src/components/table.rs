use dioxus::prelude::*;

use crate::components::common::{style_str, ClassBuilder};

// Table CSS class constants
pub const TABLE: &str = "el-table";
pub const TABLE_BORDERED: &str = "el-table--border";
pub const TABLE_STRIPED: &str = "el-table--striped";
pub const TABLE_HOVER: &str = "el-table--enable-row-hover";
pub const TABLE_HEADER: &str = "el-table__header";
pub const TABLE_BODY: &str = "el-table__body";
pub const TABLE_ROW: &str = "el-table__row";
pub const TABLE_CELL: &str = "el-table__cell";

/// Sort direction for table columns
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
    None,
}

/// Table column definition
#[derive(Clone, PartialEq)]
pub struct TableColumn {
    /// Column title
    pub title: String,
    /// Data key for the column
    pub key: String,
    /// Column width
    pub width: Option<String>,
    /// Whether the column is sortable
    pub sortable: bool,
    /// Whether the column is fixed (left/right)
    pub fixed: Option<String>,
}

/// Table row data type
pub type TableData = Vec<std::collections::HashMap<String, String>>;

/// Pre-computed header column rendering data (file-private).
///
/// Fields correspond 1:1 to the former tuple used by `header_cols`:
/// (title, width_style, sortable, asc_class, desc_class, col_key, is_active, current_order).
struct HeaderColData {
    title: String,
    width_style: String,
    sortable: bool,
    asc_class: String,
    desc_class: String,
    col_key: String,
    is_active: bool,
    current_order: SortOrder,
}

/// Pre-computed row rendering data (file-private).
///
/// Fields correspond 1:1 to the former tuple used by `row_render_data`:
/// (orig_idx, row_class, cells), where `cells` is a Vec of (cell_class, cell_value).
struct RowRenderData {
    orig_idx: usize,
    row_class: String,
    cells: Vec<(String, String)>,
}

/// Table props
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Table columns
    pub columns: Vec<TableColumn>,
    /// Table data
    pub data: TableData,
    /// Table height
    #[props(default)]
    pub height: Option<String>,
    /// Table max height
    #[props(default)]
    pub max_height: Option<String>,
    /// Whether to show header
    #[props(default = true)]
    pub show_header: bool,
    /// Whether to show border
    #[props(default = false)]
    pub border: bool,
    /// Whether to show stripe effect
    #[props(default = false)]
    pub stripe: bool,
    /// Whether to highlight current row
    #[props(default = false)]
    pub highlight_current_row: bool,
    /// Whether table is loading
    #[props(default = false)]
    pub loading: bool,
    /// Empty state text
    #[props(default = "No Data".to_string())]
    pub empty_text: String,
    /// Current sort column key
    #[props(default)]
    pub sort_key: Option<String>,
    /// Current sort order
    #[props(default = SortOrder::None)]
    pub sort_order: SortOrder,
    /// Current highlighted row index
    #[props(default)]
    pub current_row_index: Option<usize>,
    /// Row click handler
    #[props(default)]
    pub on_row_click: Option<EventHandler<usize>>,
    /// Sort change handler
    #[props(default)]
    pub on_sort_change: Option<EventHandler<(String, SortOrder)>>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A table component for displaying structured data
///
/// This component provides a table with features like sorting, row highlighting,
/// loading state, and empty state.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::table::{Table, TableColumn};
/// use std::collections::HashMap;
///
/// let columns = vec![
///     TableColumn {
///         title: "Name".to_string(),
///         key: "name".to_string(),
///         width: Some("200px".to_string()),
///         sortable: true,
///         fixed: None,
///     },
/// ];
///
/// let mut row1 = HashMap::new();
/// row1.insert("name".to_string(), "John".to_string());
///
/// let data = vec![row1];
///
/// rsx! {
///     Table {
///         columns: columns,
///         data: data,
///         stripe: true,
///     }
/// }
/// ```
#[component]
pub fn Table(props: TableProps) -> Element {
    let class_string = ClassBuilder::new("el-table")
        .add_if("el-table--border", props.border)
        .add_if("el-table--striped", props.stripe)
        .add_if(
            "el-table--highlight-current-row",
            props.highlight_current_row,
        )
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    let active_sort_key = props.sort_key.clone().unwrap_or_default();
    let active_sort_order = props.sort_order;

    // Pre-compute sorted data
    let sorted_rows: Vec<(usize, std::collections::HashMap<String, String>)> = {
        if !active_sort_key.is_empty() && active_sort_order != SortOrder::None {
            let mut indexed: Vec<(usize, &std::collections::HashMap<String, String>)> =
                props.data.iter().enumerate().collect();
            let sk = active_sort_key.clone();
            indexed.sort_by(|a, b| {
                let va = a.1.get(&sk).map(|s| s.as_str()).unwrap_or("");
                let vb = b.1.get(&sk).map(|s| s.as_str()).unwrap_or("");
                match active_sort_order {
                    SortOrder::Ascending => va.cmp(vb),
                    SortOrder::Descending => vb.cmp(va),
                    SortOrder::None => std::cmp::Ordering::Equal,
                }
            });
            indexed
                .into_iter()
                .map(|(i, row)| (i, row.clone()))
                .collect()
        } else {
            props
                .data
                .iter()
                .enumerate()
                .map(|(i, row)| (i, row.clone()))
                .collect()
        }
    };

    // Pre-compute header column data
    let header_cols: Vec<HeaderColData> = props
        .columns
        .iter()
        .map(|col| {
            let width_style = col
                .width
                .as_ref()
                .map(|w| format!("width: {};", w))
                .unwrap_or_default();
            let is_active = active_sort_key == col.key;
            let asc_class = if is_active {
                match active_sort_order {
                    SortOrder::Ascending => "sort-caret ascending is-active",
                    _ => "sort-caret ascending",
                }
            } else {
                "sort-caret ascending"
            };
            let desc_class = if is_active {
                match active_sort_order {
                    SortOrder::Descending => "sort-caret descending is-active",
                    _ => "sort-caret descending",
                }
            } else {
                "sort-caret descending"
            };
            let current_order = if is_active {
                active_sort_order
            } else {
                SortOrder::None
            };
            HeaderColData {
                title: col.title.clone(),
                width_style,
                sortable: col.sortable,
                asc_class: asc_class.to_string(),
                desc_class: desc_class.to_string(),
                col_key: col.key.clone(),
                is_active,
                current_order,
            }
        })
        .collect();

    // Pre-compute row rendering data
    let row_render_data: Vec<RowRenderData> = {
        let cur = props.current_row_index;
        let stripe = props.stripe;
        let columns_keys: Vec<String> = props.columns.iter().map(|c| c.key.clone()).collect();
        sorted_rows
            .iter()
            .map(|(orig_idx, row)| {
                let is_current = cur == Some(*orig_idx);
                let base_class = if *orig_idx % 2 == 1 && stripe {
                    "el-table__row el-table__row--striped"
                } else {
                    "el-table__row"
                };
                let row_class = if is_current {
                    format!("{} current-row", base_class)
                } else {
                    base_class.to_string()
                };
                let cells: Vec<(String, String)> = columns_keys
                    .iter()
                    .map(|key| {
                        (
                            "el-table__cell".to_string(),
                            row.get(key).cloned().unwrap_or_default(),
                        )
                    })
                    .collect();
                RowRenderData {
                    orig_idx: *orig_idx,
                    row_class,
                    cells,
                }
            })
            .collect()
    };

    let has_data = !props.data.is_empty();
    let col_count = props.columns.len();
    let empty_text = props.empty_text.clone();
    let show_header = props.show_header;
    let on_row_click = props.on_row_click;
    let on_sort_change = props.on_sort_change;
    let loading = props.loading;

    rsx! {
        div {
            class: "el-table__wrapper",
            style: "{style_string}",

            table {
                class: "{class_string}",

                if show_header {
                    thead {
                        class: "el-table__header",

                        tr {
                            class: "el-table__row",

                            for HeaderColData { title, width_style, sortable, asc_class, desc_class, col_key, is_active, current_order } in header_cols.into_iter() {
                                th {
                                    class: "el-table__cell",
                                    style: "{width_style}",

                                    onclick: move |_| {
                                        if sortable {
                                            let new_order = if is_active {
                                                match current_order {
                                                    SortOrder::None => SortOrder::Ascending,
                                                    SortOrder::Ascending => SortOrder::Descending,
                                                    SortOrder::Descending => SortOrder::None,
                                                }
                                            } else {
                                                SortOrder::Ascending
                                            };
                                            if let Some(handler) = on_sort_change.as_ref() {
                                                handler.call((col_key.clone(), new_order));
                                            }
                                        }
                                    },

                                    div {
                                        class: "cell",

                                        if sortable {
                                            span {
                                                class: "el-table__column-label",
                                                "{title}"
                                            }
                                            span {
                                                class: "caret-wrapper",
                                                i { class: "{asc_class}" }
                                                i { class: "{desc_class}" }
                                            }
                                        } else {
                                            "{title}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if has_data {
                    tbody {
                        class: "el-table__body",

                        for RowRenderData { orig_idx, row_class, cells } in row_render_data.into_iter() {
                            tr {
                                class: "{row_class}",

                                onclick: move |_| {
                                    if let Some(handler) = on_row_click.as_ref() {
                                        handler.call(orig_idx);
                                    }
                                },

                                for (cell_class, cell_value) in cells.into_iter() {
                                    td {
                                        class: "{cell_class}",
                                        div {
                                            class: "cell",
                                            "{cell_value}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    tbody {
                        class: "el-table__body",
                        tr {
                            class: "el-table__empty-row",
                            td {
                                class: "el-table__cell",
                                colspan: "{col_count}",
                                div {
                                    class: "el-table__empty-block",
                                    div {
                                        class: "el-table__empty-text",
                                        "{empty_text}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if loading {
                div {
                    class: "el-table__loading-mask",
                    div {
                        class: "el-loading-spinner",
                        i { class: "el-icon-loading" }
                        span { "Loading..." }
                    }
                }
            }
        }
    }
}

/// Data list component for simpler data display
#[derive(Props, Clone, PartialEq)]
pub struct DataListProps {
    /// List items
    pub items: Vec<String>,
    /// Whether to show loading state
    #[props(default = false)]
    pub loading: bool,
    /// Whether to show empty state
    #[props(default = true)]
    pub show_empty: bool,
    /// Empty state message
    #[props(default = "No data".to_string())]
    pub empty_text: String,
    /// List direction (vertical/horizontal)
    #[props(default = "vertical".to_string())]
    pub direction: String,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Item click handler
    #[props(default)]
    pub on_item_click: Option<EventHandler<usize>>,
}

/// A data list component for displaying item collections
#[component]
pub fn DataList(props: DataListProps) -> Element {
    let direction_class = format!("el-data-list--{}", props.direction);
    let class_string = ClassBuilder::new("el-data-list")
        .add_class(&direction_class)
        .add_if("el-data-list--loading", props.loading)
        .add_if("el-data-list--empty", props.items.is_empty())
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    if props.items.is_empty() && props.show_empty {
        return rsx! {
            div {
                class: "{class_string} el-data-list--empty-state",
                style: "{style_string}",

                div {
                    class: "el-empty",
                    div {
                        class: "el-empty__image",
                        i { class: "el-icon-document" }
                    }
                    div {
                        class: "el-empty__description",
                        p { "{props.empty_text}" }
                    }
                }
            }
        };
    }

    let on_item_click = props.on_item_click;

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            for (index, item) in props.items.iter().enumerate() {
                div {
                    class: "el-data-list__item",

                    onclick: move |_| {
                        if let Some(handler) = on_item_click.as_ref() {
                            handler.call(index);
                        }
                    },

                    "{item}"
                }
            }

            if props.loading {
                div {
                    class: "el-data-list__loading",
                    div {
                        class: "el-loading-spinner",
                        i { class: "el-icon-loading" }
                        span { "Loading..." }
                    }
                }
            }
        }
    }
}
