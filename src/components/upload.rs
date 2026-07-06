use dioxus::prelude::*;

/// Upload drag type
#[derive(Clone, PartialEq)]
pub enum UploadType {
    Select,
    Drag,
}

/// Upload props
#[derive(Props, Clone, PartialEq)]
pub struct UploadProps {
    /// Upload URL
    #[props(default)]
    pub action: Option<String>,

    /// Accepted file types
    #[props(default)]
    pub accept: Option<String>,

    /// Whether to support multiple files
    #[props(default = false)]
    pub multiple: bool,

    /// Whether to auto upload
    #[props(default = true)]
    pub auto_upload: bool,

    /// Whether to show file list
    #[props(default = true)]
    pub show_file_list: bool,

    /// Upload type
    #[props(default = UploadType::Select)]
    pub upload_type: UploadType,

    /// Maximum number of files
    #[props(default)]
    pub limit: Option<u32>,

    /// File list
    #[props(default)]
    pub file_list: Vec<String>,

    /// Button text
    #[props(default = "Upload".to_string())]
    pub button_text: String,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Upload component for file uploads
#[component]
pub fn Upload(props: UploadProps) -> Element {
    let mut class_names = vec!["el-upload".to_string()];
    match props.upload_type {
        UploadType::Drag => class_names.push("el-upload--drag".to_string()),
        UploadType::Select => class_names.push("el-upload--select".to_string()),
    }
    if let Some(ref c) = props.class { class_names.push(c.clone()); }

    rsx! {
        div {
            class: "{class_names.join(\" \")}",
            style: props.style.clone().unwrap_or_default(),
            input {
                r#type: "file",
                class: "el-upload__input",
                accept: props.accept.clone().unwrap_or_default(),
                multiple: props.multiple,
                onchange: move |e| {
                    if let Some(handler) = props.on_change {
                        handler.call(e.value());
                    }
                },
            }
            if props.upload_type == UploadType::Drag {
                div {
                    class: "el-upload-dragger",
                    i { class: "el-icon-upload" }
                    div { class: "el-upload__text", "Drop file here or click to upload" }
                }
            } else {
                button {
                    class: "el-button el-button--primary",
                    "{props.button_text}"
                }
            }
            if props.show_file_list && !props.file_list.is_empty() {
                ul {
                    class: "el-upload-list",
                    for file in props.file_list.iter() {
                        li {
                            class: "el-upload-list__item",
                            span { class: "el-upload-list__item-name", "{file}" }
                        }
                    }
                }
            }
        }
    }
}
