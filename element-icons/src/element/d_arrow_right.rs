// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn DArrowRight(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M452.9 149.3a29 29 0 0 1 41.7 0l331.6 340.4a32 32 0 0 1 0 44.6L494.6 874.6a29 29 0 0 1-41.7 0 30.6 30.6 0 0 1 0-42.7L764.7 512 453 192a30.6 30.6 0 0 1 0-42.7m-256 0a29 29 0 0 1 41.7 0l331.6 340.4a32 32 0 0 1 0 44.6L238.6 874.6a29 29 0 0 1-41.7 0 30.6 30.6 0 0 1 0-42.7L508.7 512 197 192a30.6 30.6 0 0 1 0-42.7")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
