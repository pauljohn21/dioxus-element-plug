use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PropInfo {
    pub name: String,
    pub prop_type: String,
    pub default: String,
    pub description: String,
}

impl PropInfo {
    pub fn new(name: &str, prop_type: &str, default: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            prop_type: prop_type.to_string(),
            default: default.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PropsTableProps {
    pub props: Vec<PropInfo>,
}

#[component]
pub fn PropsTable(props: PropsTableProps) -> Element {
    rsx! {
        div {
            class: "props-table",
            style: "margin: 24px 0;",

            h3 { "Props" }

            table {
                class: "el-table",
                style: "width: 100%; border-collapse: collapse;",

                thead {
                    tr {
                        style: "background: var(--el-fill-color-light);",
                        th { style: "padding: 12px; text-align: left; border-bottom: 1px solid var(--el-border-color); font-weight: 600;", "Name" }
                        th { style: "padding: 12px; text-align: left; border-bottom: 1px solid var(--el-border-color); font-weight: 600;", "Type" }
                        th { style: "padding: 12px; text-align: left; border-bottom: 1px solid var(--el-border-color); font-weight: 600;", "Default" }
                        th { style: "padding: 12px; text-align: left; border-bottom: 1px solid var(--el-border-color); font-weight: 600;", "Description" }
                    }
                }

                tbody {
                    for prop in props.props.iter() {
                        tr {
                            style: "border-bottom: 1px solid var(--el-border-color-lighter);",
                            td { style: "padding: 12px; font-family: monospace; color: var(--el-color-primary);", "{prop.name}" }
                            td { style: "padding: 12px; font-family: monospace; color: var(--el-text-color-secondary);", "{prop.prop_type}" }
                            td { style: "padding: 12px; font-family: monospace;", "{prop.default}" }
                            td { style: "padding: 12px; color: var(--el-text-color-regular);", "{prop.description}" }
                        }
                    }
                }
            }
        }
    }
}
