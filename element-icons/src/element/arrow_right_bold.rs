// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn ArrowRightBold(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M338.8 104.7a64 64 0 0 0 0 90.5L655.6 512 338.8 828.8a64 64 0 0 0 90.4 90.5l362.1-362a64 64 0 0 0 0-90.5l-362-362.1a64 64 0 0 0-90.5 0")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
