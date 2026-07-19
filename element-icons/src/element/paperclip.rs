// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Paperclip(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M602.5 240.4A192 192 0 1 1 874 512L557.2 828.8a256 256 0 0 1-362-362L602.5 59.5l45.2 45.2L240.4 512A192 192 0 0 0 512 783.6l316.8-316.8a128 128 0 1 0-181-181.1L353.5 579.9a32 32 0 1 0 45.2 45.3L693 331l45.3 45.3-294.2 294.1a96 96 0 1 1-135.7-135.7z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
