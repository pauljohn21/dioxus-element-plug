// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn BoxIcon(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M317 128 128 344v552h768V344L707 128zm-14.5-64h419a32 32 0 0 1 24 10.9l206.6 236A32 32 0 0 1 960 332v596a32 32 0 0 1-32 32H96a32 32 0 0 1-32-32V332a32 32 0 0 1 8-21L278.3 75a32 32 0 0 1 24.1-11")]),
        path(&[attr("fill", "currentColor"), attr("d", "M64 320h896v64H64z")]),
        path(&[attr("fill", "currentColor"), attr("d", "M448 327.9V640h128V327.9L526 128h-28zM448 64h128l64 256v352a32 32 0 0 1-32 32H416a32 32 0 0 1-32-32V320z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
