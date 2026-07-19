// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Goods(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M320 288v-22.3C320 154.7 405.5 64 512 64s192 90.7 192 201.7V288h131a32 32 0 0 1 31.9 28.8l57.6 576a32 32 0 0 1-31.8 35.2H131.3a32 32 0 0 1-31.8-35.2l57.6-576A32 32 0 0 1 189 288zm64 0h256v-22.3c0-76.5-57.7-137.7-128-137.7s-128 61.2-128 137.7zm-64 64H218l-51.3 512h690.6L806 352H704v96a32 32 0 1 1-64 0v-96H384v96a32 32 0 0 1-64 0z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
