// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn StarFilled(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M313.6 924.5a70 70 0 0 1-74.2-5.4 70 70 0 0 1-28-68.9l38-220.9L89 473a70.4 70.4 0 0 1 3.7-104.3A70 70 0 0 1 128 353l221.8-32.3 99.2-201a70.4 70.4 0 0 1 100.2-28.5 70 70 0 0 1 26 28.6l99.2 201 221.8 32.2a70.4 70.4 0 0 1 39 120L774.7 629.5l38 220.9a70.4 70.4 0 0 1-102.2 74.2L512 820.1z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
