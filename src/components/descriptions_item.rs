use dioxus::prelude::*;

/// DescriptionsItem props
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsItemProps {
    #[props(default)]
    pub children: Element,

    /// Label text
    pub label: String,

    /// Column span
    #[props(default = 1)]
    pub span: u32,

    /// Label width
    #[props(default)]
    pub label_width: Option<String>,

    /// Content class
    #[props(default)]
    pub class: Option<String>,
}

/// DescriptionsItem component for individual description entries
#[component]
pub fn DescriptionsItem(props: DescriptionsItemProps) -> Element {
    rsx! {
        td {
            class: "el-descriptions__label el-descriptions__cell",
            "{props.label}"
        }
        td {
            class: "el-descriptions__content el-descriptions__cell {props.class.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}
