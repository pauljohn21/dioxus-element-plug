// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Sort(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M384 96a32 32 0 0 1 64 0v786.8a32 32 0 0 1-54.6 22.6L96 608a32 32 0 0 1 0-45.3h.2a32 32 0 0 1 45.1 0l242.8 243zm192 45.2a32 32 0 0 1 54.6-22.5L928 416a32 32 0 0 1 0 45.3h-.2a32 32 0 0 1-45.1 0L640 218.5V928a32 32 0 1 1-64 0z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
