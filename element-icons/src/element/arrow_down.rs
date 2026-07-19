// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn ArrowDown(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M831.9 340.9 512 652.7 192.1 340.9a30.6 30.6 0 0 0-42.7 0 29 29 0 0 0 0 41.6l340.3 331.7a32 32 0 0 0 44.6 0l340.3-331.7a29 29 0 0 0 0-41.7 30.6 30.6 0 0 0-42.7 0z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
