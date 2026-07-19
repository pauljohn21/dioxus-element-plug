// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Bottom(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M544 805.9V168a32 32 0 1 0-64 0v637.9l-233.3-248a30.7 30.7 0 0 0-45.4 0 35.5 35.5 0 0 0 0 48.1l288 306a30.7 30.7 0 0 0 45.4 0l288-306a35.5 35.5 0 0 0 0-48 30.7 30.7 0 0 0-45.4 0z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
