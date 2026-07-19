// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Switch(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M118.7 438.7a32 32 0 0 1 0-45.3L416 96l4.5-3.8a32 32 0 0 1 40.7 3.8l3.8 4.5a32 32 0 0 1-3.8 40.8L218.6 384H928a32 32 0 1 1 0 64H141.2a32 32 0 0 1-22.5-9.3M64 608a32 32 0 0 1 32-32h786.8a32 32 0 0 1 22.6 54.6L608 928l-4.5 3.8a32 32 0 0 1-40.8-49l243-242.8H96a32 32 0 0 1-32-32")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
