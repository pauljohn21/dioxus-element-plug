use dioxus::prelude::*;
pub use super::cascader::{CascaderOption, CascaderProps};

use crate::components::common::{ClassBuilder, style_str, fire_event};

#[cfg(feature = "icons")]
use element_icons::element::ArrowRight;

/// CascaderPanel props
#[derive(Props, Clone, PartialEq)]
pub struct CascaderPanelProps {
    /// Options tree
    #[props(default)]
    pub options: Vec<CascaderOption>,

    /// Controlled selected path value (joined by separator)
    #[props(default)]
    pub model_value: Option<String>,

    /// Separator for path values
    #[props(default = " / ".to_string())]
    pub separator: String,

    /// Change event handler — receives full path values joined by separator
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Render cascader arrow icon with conditional compilation
#[cfg(feature = "icons")]
fn render_cascader_arrow() -> Element {
    rsx! {
        ArrowRight {
            style: Some("margin-left: auto;".to_string()),
        }
    }
}

/// Render cascader arrow icon fallback when icons feature is disabled
#[cfg(not(feature = "icons"))]
fn render_cascader_arrow() -> Element {
    rsx! {
        i {
            class: "el-icon-arrow-right",
            style: "margin-left: auto;"
        }
    }
}

/// CascaderPanel component — core multi-level cascading panel
#[component]
pub fn CascaderPanel(props: CascaderPanelProps) -> Element {
    // selected_path tracks the index path for each expanded level
    let mut selected_path: Signal<Vec<usize>> = use_signal(Vec::new);

    let class_string = ClassBuilder::new("el-cascader-panel")
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);
    let on_change = props.on_change;
    let separator = props.separator.clone();

    // Build the list of menus to render based on selected_path
    // Each menu is a slice of options at that level
    // We pre-extract the data for each level
    let mut levels: Vec<Vec<(String, String, bool, bool)>> = vec![];
    let mut current_options = &props.options;
    levels.push(
        current_options
            .iter()
            .map(|o| (o.value.clone(), o.label.clone(), o.disabled, !o.children.is_empty()))
            .collect(),
    );
    for &idx in selected_path().iter() {
        if let Some(opt) = current_options.get(idx) {
            if opt.children.is_empty() {
                break;
            }
            current_options = &opt.children;
            levels.push(
                current_options
                    .iter()
                    .map(|o| (o.value.clone(), o.label.clone(), o.disabled, !o.children.is_empty()))
                    .collect(),
            );
        } else {
            break;
        }
    }

    // Build path values using the pure function
    let path_values = build_path_values(&props.options, &selected_path());

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            for (level, opts) in levels.into_iter().enumerate() {
                div {
                    class: "el-cascader-menu",
                    for (idx, (value, label, disabled, has_children)) in opts.into_iter().enumerate() {
                        {
                            let is_active = selected_path().get(level) == Some(&idx);
                            let sep = separator.clone();
                            let path_vals = path_values.clone();
                            let current_val = value.clone();
                            let current_lbl = label.clone();
                            let current_disabled = disabled;
                            let current_has_children = has_children;
                            let current_level = level;
                            let current_idx = idx;

                            rsx! {
                                div {
                                    class: "el-cascader-menu__item",
                                    class: if is_active { "is-active" },
                                    class: if current_disabled { "is-disabled" },
                                    onclick: move |_| {
                                        if current_disabled {
                                            return;
                                        }
                                        if current_has_children {
                                            // Expand: update selected_path up to this level
                                            let mut new_path = selected_path();
                                            if new_path.len() > current_level {
                                                new_path.truncate(current_level);
                                            }
                                            new_path.push(current_idx);
                                            selected_path.set(new_path);
                                        } else {
                                            // Leaf node: fire on_change with full path
                                            let mut full_path = path_vals.clone();
                                            // Truncate to current level
                                            if full_path.len() > current_level {
                                                full_path.truncate(current_level);
                                            }
                                            full_path.push(current_val.clone());
                                            let joined = full_path.join(&sep);
                                            fire_event(&on_change, joined);
                                        }
                                    },
                                    "{current_lbl}"
                                    if current_has_children {
                                        {render_cascader_arrow()}
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

/// Build the full path values from options and selected_path indices.
/// This is a pure function for testing.
fn build_path_values(options: &[CascaderOption], path: &[usize]) -> Vec<String> {
    let mut result = vec![];
    let mut current = options;
    for &idx in path {
        if let Some(opt) = current.get(idx) {
            result.push(opt.value.clone());
            current = &opt.children;
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::cascader::CascaderOption;

    #[test]
    fn test_build_path_values_single_level() {
        let options = vec![
            CascaderOption::new("zj", "Zhejiang"),
            CascaderOption::new("js", "Jiangsu"),
        ];
        let path = vec![0];
        let values = build_path_values(&options, &path);
        assert_eq!(values, vec!["zj"]);
    }

    #[test]
    fn test_build_path_values_multi_level() {
        let options = vec![
            CascaderOption::new("zj", "Zhejiang")
                .child(CascaderOption::new("hz", "Hangzhou"))
                .child(CascaderOption::new("nb", "Ningbo")),
            CascaderOption::new("js", "Jiangsu")
                .child(CascaderOption::new("nj", "Nanjing")),
        ];
        let path = vec![0, 1];
        let values = build_path_values(&options, &path);
        assert_eq!(values, vec!["zj", "nb"]);
    }

    #[test]
    fn test_build_path_values_empty() {
        let options: Vec<CascaderOption> = vec![];
        let path: Vec<usize> = vec![];
        let values = build_path_values(&options, &path);
        assert!(values.is_empty());
    }

    #[test]
    fn test_build_path_values_invalid_index() {
        let options = vec![CascaderOption::new("a", "A")];
        let path = vec![5]; // invalid index
        let values = build_path_values(&options, &path);
        assert!(values.is_empty());
    }

    #[test]
    fn test_path_join() {
        let values = ["zj".to_string(), "hz".to_string()];
        let joined = values.join(" / ");
        assert_eq!(joined, "zj / hz");
    }
}
