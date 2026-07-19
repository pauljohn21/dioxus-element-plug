// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Location(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M800 416a288 288 0 1 0-576 0c0 118.1 94.5 272.1 288 456.6C705.5 688 800 534 800 416M512 960Q160 640 160 416a352 352 0 0 1 704 0q0 224-352 544")]),
        path(&[attr("fill", "currentColor"), attr("d", "M512 512a96 96 0 1 0 0-192 96 96 0 0 0 0 192m0 64a160 160 0 1 1 0-320 160 160 0 0 1 0 320")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
