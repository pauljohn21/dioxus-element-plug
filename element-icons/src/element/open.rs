// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Open(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M330 257.1a254.9 254.9 0 0 0 0 509.8h364a254.9 254.9 0 0 0 0-509.8zm0-72.8h364a327.7 327.7 0 1 1 0 655.4H330a327.7 327.7 0 1 1 0-655.4")]),
        path(&[attr("fill", "currentColor"), attr("d", "M694 621.2a109.2 109.2 0 1 0 0-218.4 109.2 109.2 0 0 0 0 218.4m0 72.8a182 182 0 1 1 0-364 182 182 0 0 1 0 364")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
