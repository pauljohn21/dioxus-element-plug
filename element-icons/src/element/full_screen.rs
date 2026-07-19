// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn FullScreen(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "m160 96 192 .3a32 32 0 0 1 0 64l-192-.2V352a32 32 0 0 1-64 0V96zm0 832H96V672a32 32 0 1 1 64 0v192l192-.3a32 32 0 1 1 0 64zM864 96h64v256a32 32 0 1 1-64 0V160l-192 .3a32 32 0 1 1 0-64zm0 832-192-.3a32 32 0 0 1 0-64l192 .2V672a32 32 0 1 1 64 0v256z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
