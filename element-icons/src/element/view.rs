// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn View(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M512 160c320 0 512 352 512 352S832 864 512 864 0 512 0 512s192-352 512-352m0 64c-225.3 0-384.1 208-436.8 288 52.6 79.9 211.5 288 436.8 288s384.1-208 436.8-288C896.2 432.1 737.3 224 512 224m0 64a224 224 0 1 1 0 448 224 224 0 0 1 0-448m0 64a160 160 0 0 0-160 160c0 88.2 71.7 160 160 160s160-71.8 160-160-71.7-160-160-160")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
