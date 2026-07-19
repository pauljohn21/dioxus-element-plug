// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Key(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M448 456V96a32 32 0 0 1 32-32h192a32 32 0 0 1 0 64H512v128h160a32 32 0 0 1 0 64H512v128a256 256 0 1 1-64 8m64 440a192 192 0 1 0 0-384 192 192 0 0 0 0 384")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
