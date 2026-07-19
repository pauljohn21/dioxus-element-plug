// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn CircleCloseFilled(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M512 64a448 448 0 1 1 0 896 448 448 0 0 1 0-896m0 393.7L408 353.6a38.4 38.4 0 1 0-54.4 54.3l104 104.1-104 104a38.4 38.4 0 1 0 54.3 54.4l104.1-104 104 104a38.4 38.4 0 1 0 54.4-54.3L566.4 512l104-104a38.4 38.4 0 1 0-54.3-54.4z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
