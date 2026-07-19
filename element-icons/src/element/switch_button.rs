// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn SwitchButton(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M352 159.9v70.5a352 352 0 1 0 320 0v-70.5A416.1 416.1 0 0 1 512 960a416 416 0 0 1-160-800.1")]),
        path(&[attr("fill", "currentColor"), attr("d", "M512 64q32 0 32 32v320q0 32-32 32t-32-32V96q0-32 32-32")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
