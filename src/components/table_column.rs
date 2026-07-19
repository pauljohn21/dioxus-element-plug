use dioxus::prelude::*;

/// TableColumn props
///
/// TableColumn is a **declarative component** — it does not render any DOM.
/// Instead, it defines column metadata that is consumed by the parent
/// [`Table`](super::table::Table) component via its `columns: Vec<TableColumn>`
/// prop.
///
/// # Usage
///
/// ```rust,ignore
/// rsx! {
///     Table {
///         columns: vec![
///             TableColumn {
///                 label: Some("Name".to_string()),
///                 prop: Some("name".to_string()),
///                 width: Some("200px".to_string()),
///                 sortable: true,
///             },
///             TableColumn {
///                 label: Some("Age".to_string()),
///                 prop: Some("age".to_string()),
///             },
///         ],
///         data: rows,
///     }
/// }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct TableColumnProps {
    /// Column label (header text)
    #[props(default)]
    pub label: Option<String>,

    /// Column prop/field name
    #[props(default)]
    pub prop: Option<String>,

    /// Column width
    #[props(default)]
    pub width: Option<String>,

    /// Minimum column width
    #[props(default)]
    pub min_width: Option<String>,

    /// Whether the column is fixed
    #[props(default)]
    pub fixed: Option<String>,

    /// Whether the column is sortable
    #[props(default = false)]
    pub sortable: bool,

    /// Column type (selection, index, expand)
    #[props(default)]
    pub column_type: Option<String>,

    /// Whether the column is resizable
    #[props(default = true)]
    pub resizable: bool,

    /// Text alignment
    #[props(default = "left".to_string())]
    pub align: String,

    /// Header alignment
    #[props(default)]
    pub header_align: Option<String>,

    /// Whether to show overflow tooltip
    #[props(default = false)]
    pub show_overflow_tooltip: bool,

    #[props(default)]
    pub class: Option<String>,
}

/// TableColumn — declarative column definition for Table.
///
/// This component renders nothing. It exists purely to provide a ergonomic
/// builder API for constructing `TableColumn` structs. Use it in conjunction
/// with the `Table` component's `columns` prop.
#[component]
pub fn TableColumn(props: TableColumnProps) -> Element {
    // TableColumn is a declarative component — it renders no DOM.
    // Column metadata is passed to Table via the `columns` Vec.
    let _ = props;
    rsx! {}
}
