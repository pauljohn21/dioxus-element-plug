// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn PieChart(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M448 68.5v64.8A384.1 384.1 0 0 0 512 896a384 384 0 0 0 378.7-320h64.8A448.1 448.1 0 0 1 64 512 448 448 0 0 1 448 68.5")]),
        path(&[attr("fill", "currentColor"), attr("d", "M576 97.3V448h350.7A384 384 0 0 0 576 97.3M512 64V33.2A448 448 0 0 1 990.8 512H512z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
