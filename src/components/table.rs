use dioxus::prelude::*;

// Table CSS class constants
pub const TABLE: &str = "el-table";
pub const TABLE_BORDERED: &str = "el-table--border";
pub const TABLE_STRIPED: &str = "el-table--striped";
pub const TABLE_HOVER: &str = "el-table--enable-row-hover";
pub const TABLE_HEADER: &str = "el-table__header";
pub const TABLE_BODY: &str = "el-table__body";
pub const TABLE_ROW: &str = "el-table__row";
pub const TABLE_CELL: &str = "el-table__cell";

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

/// Table row data type - simplified
pub type TableData = Vec<std::collections::HashMap<String, String>>;

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
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A table component for displaying structured data
///
/// This component provides a basic table with features like sorting and styling.
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
///     TableColumn {
///         title: "Age".to_string(),
///         key: "age".to_string(),
///         width: None,
///         sortable: false,
///         fixed: None,
///     },
/// ];
///
/// let mut row1 = HashMap::new();
/// row1.insert("name".to_string(), "John".to_string());
/// row1.insert("age".to_string(), "30".to_string());
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
    let mut class_names = vec!["el-table".to_string()];
    
    if props.border {
        class_names.push("el-table--border".to_string());
    }
    
    if props.stripe {
        class_names.push("el-table--striped".to_string());
    }
    
    if props.highlight_current_row {
        class_names.push("el-table--highlight-current-row".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let _class_string = class_names.join(" ");
    let _style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    let class_string = if props.border { "el-table el-table--border" } else { "el-table" };
    let stripe_class = if props.stripe { " el-table--striped" } else { "" };
    let full_class = format!("{}{}", class_string, stripe_class);

    rsx! {
        div {
            class: "el-table__wrapper",
            
            if let Some(height) = &props.height {
                div {
                    style: "max-height: {height}; overflow-y: auto;",
                    table {
                        class: "{full_class}",
                        {build_table_content(&props)}
                    }
                }
            } else {
                table {
                    class: "{full_class}",
                    {build_table_content(&props)}
                }
            }
        }
    }
}

fn build_table_content(props: &TableProps) -> Element {
    rsx! {
        if props.show_header {
            thead {
                class: "el-table__header",
                
                tr {
                    class: "el-table__row",
                    
                    for column in props.columns.iter() {
                        th {
                            class: "el-table__cell",
                            style: column.width.as_ref().map(|w| format!("width: {};", w)).unwrap_or_default(),
                            
                            div {
                                class: "cell",
                                
                                if column.sortable {
                                    span {
                                        "{column.title}"
                                    }
                                    span {
                                        class: "caret-wrapper",
                                        
                                        i {
                                            class: "sort-caret ascending"
                                        }
                                        
                                        i {
                                            class: "sort-caret descending"
                                        }
                                    }
                                } else {
                                    "{column.title}"
                                }
                            }
                        }
                    }
                }
            }
        }
        
        tbody {
            class: "el-table__body",
            
            for (row_index, row_data) in props.data.iter().enumerate() {
                tr {
                    class: if row_index % 2 == 1 && props.stripe {
                        "el-table__row el-table__row--striped"
                    } else {
                        "el-table__row"
                    },
                    
                    for column in props.columns.iter() {
                        td {
                            class: "el-table__cell",
                            
                            div {
                                class: "cell",
                                "{row_data.get(&column.key).unwrap_or(&String::new())}"
                            }
                        }
                    }
                }
            }
        }
        
        if props.loading {
            div {
                class: "el-table__loading",
                
                div {
                    class: "el-loading-spinner",
                    
                    i {
                        class: "el-icon-loading"
                    }
                    
                    span {
                        "Loading..."
                    }
                }
            }
        }
    }
}

#[component]
fn table_content(props: TableProps) -> Element {
    let class_string = if props.border { "el-table el-table--border" } else { "el-table" };
    let stripe_class = if props.stripe { " el-table--striped" } else { "" };
    let full_class = format!("{}{}", class_string, stripe_class);

    rsx! {
        table {
            class: "{full_class}",
            style: props.style.as_ref().cloned().unwrap_or_default(),
            
            if props.show_header {
                thead {
                    class: "el-table__header",
                    
                    tr {
                        class: "el-table__row",
                        
                        for column in props.columns.iter() {
                            th {
                                class: "el-table__cell",
                                style: column.width.as_ref().map(|w| format!("width: {};", w)).unwrap_or_default(),
                                
                                div {
                                    class: "cell",
                                    
                                    if column.sortable {
                                        span {
                                            "{column.title}"
                                        }
                                        span {
                                            class: "caret-wrapper",
                                            
                                            i {
                                                class: "sort-caret ascending"
                                            }
                                            
                                            i {
                                                class: "sort-caret descending"
                                            }
                                        }
                                    } else {
                                        "{column.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            tbody {
                class: "el-table__body",
                
                for (row_index, row_data) in props.data.iter().enumerate() {
                    tr {
                        class: if row_index % 2 == 1 && props.stripe {
                            "el-table__row el-table__row--striped"
                        } else {
                            "el-table__row"
                        },
                        
                        for column in props.columns.iter() {
                            td {
                                class: "el-table__cell",
                                
                                div {
                                    class: "cell",
                                    "{row_data.get(&column.key).unwrap_or(&String::new())}"
                                }
                            }
                        }
                    }
                }
            }
            
            if props.loading {
                div {
                    class: "el-table__loading",
                    
                    div {
                        class: "el-loading-spinner",
                        
                        i {
                            class: "el-icon-loading"
                        }
                        
                        span {
                            "Loading..."
                        }
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
///
/// This component provides a flexible way to display collections of data
/// with custom templates and interaction handlers.
#[component]
pub fn DataList(props: DataListProps) -> Element {
    let mut class_names = vec!["el-data-list".to_string()];
    
    class_names.push(format!("el-data-list--{}", props.direction));
    
    if props.loading {
        class_names.push("el-data-list--loading".to_string());
    }
    
    if props.items.is_empty() {
        class_names.push("el-data-list--empty".to_string());
    }
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let style_string = props.style.as_ref().cloned().unwrap_or_default();
    
    if props.items.is_empty() && props.show_empty {
        return rsx! {
            div {
                class: "{class_string} el-data-list--empty-state",
                style: "{style_string}",
                
                div {
                    class: "el-empty",
                    
                    div {
                        class: "el-empty__image",
                        i {
                            class: "el-icon-document"
                        }
                    }
                    
                    div {
                        class: "el-empty__description",
                        p {
                            "{props.empty_text}"
                        }
                    }
                }
            }
        };
    }
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            
            for (index, item) in props.items.iter().enumerate() {
                div {
                    class: "el-data-list__item",
                    
                    onclick: move |_| {
                        if let Some(handler) = props.on_item_click {
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
                        
                        i {
                            class: "el-icon-loading"
                        }
                        
                        span {
                            "Loading..."
                        }
                    }
                }
            }
        }
    }
}