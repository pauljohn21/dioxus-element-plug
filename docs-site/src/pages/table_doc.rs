use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use std::collections::HashMap;

use crate::components::props_table::{PropInfo, PropsTable};
use crate::components::code_block::CodeBlock;
use crate::components::demo_container::DemoContainer;

#[component]
pub fn TableDocPage() -> Element {
    let columns = vec![
        TableColumn {
            title: "Name".to_string(),
            key: "name".to_string(),
            width: None,
            sortable: true,
            fixed: None,
        },
        TableColumn {
            title: "Age".to_string(),
            key: "age".to_string(),
            width: Some("100px".to_string()),
            sortable: true,
            fixed: None,
        },
        TableColumn {
            title: "Address".to_string(),
            key: "address".to_string(),
            width: None,
            sortable: false,
            fixed: None,
        },
    ];

    let data = vec![
        {
            let mut row = HashMap::new();
            row.insert("name".to_string(), "John Doe".to_string());
            row.insert("age".to_string(), "28".to_string());
            row.insert("address".to_string(), "New York".to_string());
            row
        },
        {
            let mut row = HashMap::new();
            row.insert("name".to_string(), "Jane Smith".to_string());
            row.insert("age".to_string(), "32".to_string());
            row.insert("address".to_string(), "London".to_string());
            row
        },
        {
            let mut row = HashMap::new();
            row.insert("name".to_string(), "Bob Johnson".to_string());
            row.insert("age".to_string(), "24".to_string());
            row.insert("address".to_string(), "Paris".to_string());
            row
        },
    ];

    rsx! {
        div {
            h1 { "Table" }

            p {
                style: "color: var(--el-text-color-secondary); font-size: 16px;",
                "Display structured data with sorting and selection."
            }

            // Basic example
            h2 { "Basic Usage" }

            DemoContainer {
                title: "Basic Table",
                description: Some("Table with columns and data".to_string()),

                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                }
            }

            CodeBlock {
                code: r#"let columns = vec![
    TableColumn {
        title: "Name".to_string(),
        key: "name".to_string(),
        width: None,
        sortable: true,
        fixed: None,
    },
    TableColumn {
        title: "Age".to_string(),
        key: "age".to_string(),
        width: Some("100px".to_string()),
        sortable: true,
        fixed: None,
    },
];

let data = vec![
    HashMap::from([
        ("name".to_string(), "John".to_string()),
        ("age".to_string(), "28".to_string()),
    ]),
];

rsx! {
    Table {
        columns: columns,
        data: data,
    }
}""#.to_string(),
                language: Some("rust".to_string()),
            }

            // Striped
            h2 { "Striped" }

            DemoContainer {
                title: "Striped Table",
                description: Some("Alternating row colors".to_string()),

                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                    stripe: true,
                }
            }

            // Border
            h2 { "Border" }

            DemoContainer {
                title: "Bordered Table",
                description: Some("Table with borders".to_string()),

                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                    border: true,
                }
            }

            // Combined
            h2 { "Striped & Border" }

            DemoContainer {
                title: "Striped and Bordered",
                description: Some("Both stripe and border enabled".to_string()),

                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                    stripe: true,
                    border: true,
                }
            }

            // Props table
            PropsTable {
                props: vec![
                    PropInfo::new("columns", "Vec<TableColumn>", "required", "Column definitions"),
                    PropInfo::new("data", "Vec<HashMap<String, String>>", "required", "Row data"),
                    PropInfo::new("stripe", "bool", "false", "Alternating row colors"),
                    PropInfo::new("border", "bool", "false", "Show borders"),
                    PropInfo::new("highlight_current_row", "bool", "false", "Highlight current row"),
                    PropInfo::new("sort_key", "Option<String>", "None", "Current sort column"),
                    PropInfo::new("sort_order", "SortOrder", "None", "Sort direction"),
                    PropInfo::new("on_sort_change", "Option<EventHandler<(String, SortOrder)>>", "None", "Sort change handler"),
                    PropInfo::new("on_row_click", "Option<EventHandler<usize>>", "None", "Row click handler"),
                ],
            }
        }
    }
}
