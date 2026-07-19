use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ApiItem {
    pub name: String,
    pub description: String,
    pub prop_type: String,
    pub default: String,
}

impl ApiItem {
    pub fn new(name: &str, desc: &str, prop_type: &str, default: &str) -> Self {
        Self {
            name: name.to_string(),
            description: desc.to_string(),
            prop_type: prop_type.to_string(),
            default: default.to_string(),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ApiTableProps {
    pub title: String,
    pub headers: Vec<String>,
    pub items: Vec<ApiItem>,
}

#[component]
pub fn ApiTable(props: ApiTableProps) -> Element {
    rsx! {
        div {
            style: "margin: 32px 0;",

            h3 {
                style: "margin: 0 0 16px 0; font-size: 18px; color: var(--el-text-color-primary);",
                "{props.title}"
            }

            div {
                style: "overflow-x: auto;",

                table {
                    style: "width: 100%; border-collapse: collapse; font-size: 14px;",

                    thead {
                        tr {
                            style: "background: var(--el-fill-color-light);",
                            for header in props.headers.iter() {
                                th {
                                    style: "padding: 12px 16px; text-align: left; border-bottom: 1px solid var(--el-border-color); color: var(--el-text-color-primary); font-weight: 600; white-space: nowrap;",
                                    "{header}"
                                }
                            }
                        }
                    }

                    tbody {
                        for item in props.items.iter() {
                            tr {
                                style: "border-bottom: 1px solid var(--el-border-color-lighter);",
                                td {
                                    style: "padding: 12px 16px; color: var(--el-color-primary); font-family: monospace;",
                                    "{item.name}"
                                }
                                td {
                                    style: "padding: 12px 16px; color: var(--el-text-color-regular); line-height: 1.6;",
                                    "{item.description}"
                                }
                                td {
                                    style: "padding: 12px 16px; color: var(--el-color-success); font-family: monospace;",
                                    "{item.prop_type}"
                                }
                                td {
                                    style: "padding: 12px 16px; color: var(--el-text-color-secondary); font-family: monospace;",
                                    "{item.default}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
