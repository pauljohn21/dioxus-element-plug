// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn CircleClose(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "m466.8 512-90.5-90.5a32 32 0 0 1 45.2-45.2l90.5 90.5 90.5-90.5a32 32 0 1 1 45.2 45.2L557.2 512l90.5 90.5a32 32 0 1 1-45.2 45.2L512 557.2l-90.5 90.5a32 32 0 0 1-45.2-45.2z")]),
        path(&[attr("fill", "currentColor"), attr("d", "M512 896a384 384 0 1 0 0-768 384 384 0 0 0 0 768m0 64a448 448 0 1 1 0-896 448 448 0 0 1 0 896")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
