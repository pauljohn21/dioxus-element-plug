// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn ArrowUp(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M488.8 344.3 149 701a32 32 0 0 0 0 44.2l.4.3a29.4 29.4 0 0 0 42.7 0l320-335.8 319.8 335.8a29.4 29.4 0 0 0 42.7 0l.4-.3a32 32 0 0 0 0-44.2L535.2 344.3a32 32 0 0 0-46.4 0")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
