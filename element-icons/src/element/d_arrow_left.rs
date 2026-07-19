// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn DArrowLeft(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M529.4 149.4a29 29 0 0 1 41.7 0 30.6 30.6 0 0 1 0 42.7L259.3 511.9 571 832a30.6 30.6 0 0 1-.5 43.2 29 29 0 0 1-41.2-.5L197.8 534.3a32 32 0 0 1 0-44.7zm256 0a29 29 0 0 1 41.7 0 30.6 30.6 0 0 1 0 42.7L515.3 511.9 827 832a30.6 30.6 0 0 1-.5 43.2 29 29 0 0 1-41.2-.5L453.8 534.3a32 32 0 0 1 0-44.7z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
