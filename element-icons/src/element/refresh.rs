// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Refresh(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M771.8 794.9A384 384 0 0 1 128 512h64a320 320 0 0 0 555.7 216.4h-93a32 32 0 1 1 0-64h149a32 32 0 0 1 32 32v149a32 32 0 1 1-64 0zM276.3 295.6h93a32 32 0 0 1 0 64H220.2a32 32 0 0 1-32-32v-149a32 32 0 0 1 64 0V229a384 384 0 0 1 644 282.9h-64a320 320 0 0 0-555.8-216.4z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
