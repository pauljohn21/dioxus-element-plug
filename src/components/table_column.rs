use dioxus::prelude::*;

/// TableColumn props
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

/// TableColumn component for defining table columns
#[component]
#[allow(unused_variables)]
pub fn TableColumn(props: TableColumnProps) -> Element {
    // TableColumn is a declarative component, it doesn't render directly
    rsx! {}
}
