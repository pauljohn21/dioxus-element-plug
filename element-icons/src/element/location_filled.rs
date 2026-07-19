// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn LocationFilled(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M512 928c24 0 117.5-68.4 192-153.2 99.5-113 160-239 160-358.8 0-189.6-155.8-320-352-320S160 226.4 160 416c0 120.3 60.5 246.4 160 359.2C394.3 859.8 488 928 512 928m0-435.2a64 64 0 1 0 0-128 64 64 0 0 0 0 128m0 140.8a204.8 204.8 0 1 1 0-409.6 204.8 204.8 0 0 1 0 409.6")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
