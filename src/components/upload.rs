use crate::components::common::{fire_event, style_str, ClassBuilder};
use dioxus::prelude::*;

/// Upload type variants
#[derive(Clone, PartialEq, Debug, Default)]
pub enum UploadType {
    #[default]
    Select,
    Drag,
}

/// File list item status
#[derive(Clone, PartialEq, Debug)]
pub enum UploadStatus {
    Ready,
    Uploading,
    Success,
    Error,
}

impl UploadStatus {
    pub fn as_class(&self) -> &'static str {
        match self {
            UploadStatus::Ready => "is-ready",
            UploadStatus::Uploading => "is-uploading",
            UploadStatus::Success => "is-success",
            UploadStatus::Error => "is-error",
        }
    }
}

/// Upload file item
#[derive(Clone, PartialEq, Debug)]
pub struct UploadFile {
    pub name: String,
    pub size: u64,
    pub status: UploadStatus,
    pub progress: u8, // 0-100
}

impl UploadFile {
    pub fn new(name: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
            status: UploadStatus::Ready,
            progress: 0,
        }
    }

    pub fn with_status(mut self, status: UploadStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = progress;
        self
    }

    /// Format file size for display
    pub fn formatted_size(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Upload props
#[derive(Props, Clone, PartialEq)]
pub struct UploadProps {
    /// Upload action URL (for actual HTTP upload)
    #[props(default)]
    pub action: Option<String>,

    /// Accepted file types (e.g., ".jpg,.png,.pdf")
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

    /// Upload type (select or drag)
    #[props(default = UploadType::Select)]
    pub upload_type: UploadType,

    /// Maximum number of files
    #[props(default)]
    pub limit: Option<u32>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether to show upload button
    #[props(default = true)]
    pub show_button: bool,

    /// Button text
    #[props(default = "点击上传".to_string())]
    pub button_text: String,

    /// Drag area text
    #[props(default = "将文件拖到此处，或点击上传".to_string())]
    pub drag_text: String,

    /// Tip text below upload area
    #[props(default)]
    pub tip: Option<String>,

    /// File list (controlled)
    #[props(default)]
    pub file_list: Vec<UploadFile>,

    /// Change event handler (called when files are selected)
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<UploadFile>>>,

    /// Remove event handler (called when file is removed)
    #[props(default)]
    pub on_remove: Option<EventHandler<String>>,

    /// Preview event handler
    #[props(default)]
    pub on_preview: Option<EventHandler<String>>,

    /// Exceed limit event handler
    #[props(default)]
    pub on_exceed: Option<EventHandler<()>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Upload component for file uploads
///
/// Provides a file upload interface with drag & drop support,
/// file list management, and progress indication.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::upload::{Upload, UploadType, UploadFile};
///
/// rsx! {
///     Upload {
///         upload_type: UploadType::Select,
///         multiple: true,
///         accept: Some(".jpg,.png".to_string()),
///         button_text: "Upload Images".to_string(),
///         on_change: move |files: Vec<UploadFile>| println!("Selected {} files", files.len()),
///     }
/// }
/// ```
#[component]
pub fn Upload(props: UploadProps) -> Element {
    let mut file_list = use_signal(|| props.file_list.clone());
    let mut is_drag_over = use_signal(|| false);

    // Update file_list when props.file_list changes
    use_effect(move || {
        file_list.set(props.file_list.clone());
    });

    // Build class string
    let class_string = ClassBuilder::new("el-upload")
        .add_class(match props.upload_type {
            UploadType::Drag => "el-upload--drag",
            UploadType::Select => "el-upload--select",
        })
        .add_if("is-disabled", props.disabled)
        .add_opt(props.class.as_ref())
        .build();

    let style_string = style_str(&props.style);

    let on_change = props.on_change;
    let on_remove = props.on_remove;
    let on_preview = props.on_preview;
    let on_exceed = props.on_exceed;

    // Handle file selection
    let mut handle_file_select = move |files: Vec<UploadFile>| {
        // Check limit
        if let Some(limit) = props.limit {
            let current_count = file_list().len();
            if current_count + files.len() > limit as usize {
                fire_event(&on_exceed, ());
                return;
            }
        }

        let mut new_list = file_list();
        new_list.extend(files);
        file_list.set(new_list.clone());
        fire_event(&on_change, new_list);
    };

    // Handle file removal
    let mut handle_remove = move |file_name: String| {
        let new_list: Vec<UploadFile> = file_list()
            .into_iter()
            .filter(|f| f.name != file_name)
            .collect();
        file_list.set(new_list.clone());
        fire_event(&on_remove, file_name);
        fire_event(&on_change, new_list);
    };

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",

            // Upload input (hidden)
            input {
                r#type: "file",
                class: "el-upload__input",
                accept: props.accept.clone().unwrap_or_default(),
                multiple: props.multiple,
                disabled: props.disabled,
                onchange: move |e: FormEvent| {
                    // Parse file info from the event
                    // Note: Actual file handling would require web_sys for file access
                    // This is a simulation for the component interface
                    let value = e.value();
                    let file_names: Vec<&str> = value.split(',').collect();
                    let files: Vec<UploadFile> = file_names
                        .iter()
                        .filter(|n| !n.is_empty())
                        .map(|name| UploadFile::new(*name, 0))
                        .collect();
                    if !files.is_empty() {
                        handle_file_select(files);
                    }
                },
            }

            // Upload trigger area
            if props.show_button {
                match props.upload_type {
                    UploadType::Drag => {
                        let drag_class = if is_drag_over() {
                            "el-upload-dragger is-dragover"
                        } else {
                            "el-upload-dragger"
                        };
                        rsx! {
                            div {
                                class: "{drag_class}",
                                ondragover: move |e: DragEvent| {
                                    e.prevent_default();
                                    if !props.disabled {
                                        is_drag_over.set(true);
                                    }
                                },
                                ondragleave: move |e: DragEvent| {
                                    e.prevent_default();
                                    is_drag_over.set(false);
                                },
                                ondrop: move |e: DragEvent| {
                                    e.prevent_default();
                                    is_drag_over.set(false);
                                    if !props.disabled {
                                        // Handle dropped files
                                        // Note: Actual file handling would require web_sys
                                        fire_event(&on_change, vec![]);
                                    }
                                },
                                i { class: "el-icon-upload" }
                                div { class: "el-upload__text", "{props.drag_text}" }
                            }
                        }
                    }
                    UploadType::Select => {
                        rsx! {
                            button {
                                class: "el-button el-button--primary",
                                disabled: props.disabled,
                                i { class: "el-icon-upload" }
                                "{props.button_text}"
                            }
                        }
                    }
                }
            }

            // Tip text
            if let Some(ref tip_text) = props.tip {
                div {
                    class: "el-upload__tip",
                    "{tip_text}"
                }
            }

            // File list
            if props.show_file_list && !file_list().is_empty() {
                ul {
                    class: "el-upload-list el-upload-list--text",
                            for file in file_list().into_iter() {
                                {
                                    let file_name = file.name.clone();
                                    let file_name_for_preview = file_name.clone();
                                    let file_name_for_remove = file_name.clone();
                                    let file_name_for_display = file_name.clone();
                                    let file_size = file.formatted_size();
                                    let status_class = file.status.as_class();
                                    let progress = file.progress;
                                    rsx! {
                                        li {
                                            class: "el-upload-list__item {status_class}",
                                            onclick: move |_| {
                                                fire_event(&on_preview, file_name_for_preview.clone());
                                            },
                                            // File info
                                            div {
                                                class: "el-upload-list__item-info",
                                                i { class: "el-icon-document" }
                                                span {
                                                    class: "el-upload-list__item-name",
                                                    "{file_name_for_display}"
                                                }
                                                span {
                                                    class: "el-upload-list__item-size",
                                                    " ({file_size})"
                                                }
                                            }
                                            // Progress bar (if uploading)
                                            if file.status == UploadStatus::Uploading {
                                                div {
                                                    class: "el-upload-list__item-progress",
                                                    div {
                                                        class: "el-progress el-progress--line",
                                                        div {
                                                            class: "el-progress-bar",
                                                            div {
                                                                class: "el-progress-bar__outer",
                                                                style: "height: 2px;",
                                                                div {
                                                                    class: "el-progress-bar__inner",
                                                                    style: "width: {progress}%;",
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // Status label
                                            if file.status == UploadStatus::Success {
                                                i { class: "el-icon-circle-check el-upload-list__item-status-label" }
                                            }
                                            if file.status == UploadStatus::Error {
                                                i { class: "el-icon-circle-close el-upload-list__item-status-label" }
                                            }
                                            // Remove button
                                            button {
                                                class: "el-upload-list__item-delete",
                                                onclick: move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    handle_remove(file_name_for_remove.clone());
                                                },
                                                i { class: "el-icon-close" }
                                            }
                                        }
                                    }
                                }
                            }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_status_as_class() {
        assert_eq!(UploadStatus::Ready.as_class(), "is-ready");
        assert_eq!(UploadStatus::Uploading.as_class(), "is-uploading");
        assert_eq!(UploadStatus::Success.as_class(), "is-success");
        assert_eq!(UploadStatus::Error.as_class(), "is-error");
    }

    #[test]
    fn test_upload_file() {
        let file = UploadFile::new("test.jpg", 1024);
        assert_eq!(file.name, "test.jpg");
        assert_eq!(file.size, 1024);
        assert_eq!(file.status, UploadStatus::Ready);
        assert_eq!(file.progress, 0);

        let file_with_status = UploadFile::new("test.jpg", 1024)
            .with_status(UploadStatus::Success)
            .with_progress(100);
        assert_eq!(file_with_status.status, UploadStatus::Success);
        assert_eq!(file_with_status.progress, 100);
    }

    #[test]
    fn test_upload_file_formatted_size() {
        let file_b = UploadFile::new("test.txt", 500);
        assert_eq!(file_b.formatted_size(), "500 B");

        let file_kb = UploadFile::new("test.txt", 1024);
        assert_eq!(file_kb.formatted_size(), "1.0 KB");

        let file_mb = UploadFile::new("test.txt", 1024 * 1024);
        assert_eq!(file_mb.formatted_size(), "1.0 MB");

        let file_gb = UploadFile::new("test.txt", 1024 * 1024 * 1024);
        assert_eq!(file_gb.formatted_size(), "1.0 GB");
    }

    #[test]
    fn test_upload_type_default() {
        assert_eq!(UploadType::default(), UploadType::Select);
    }
}
