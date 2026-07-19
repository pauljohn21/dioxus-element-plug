// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Place(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M512 512a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512")]),
        path(&[attr("fill", "currentColor"), attr("d", "M512 512a32 32 0 0 1 32 32v256a32 32 0 1 1-64 0V544a32 32 0 0 1 32-32")]),
        path(&[attr("fill", "currentColor"), attr("d", "M384 649v65c-114.2 18.4-192 57.9-192 86 0 37.7 139.9 96 320 96s320-58.3 320-96c0-28.2-77.8-67.6-192-86v-65c149.1 22 256 81.4 256 151 0 88.3-171.9 160-384 160s-384-71.7-384-160c0-69.7 106.9-129 256-151")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
