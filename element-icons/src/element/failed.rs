// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Failed(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M557.2 608 693 472.3 647.7 427 512.1 562.8 376.3 427 331 472.3 466.8 608 331 743.7l45.2 45.3L512 653.2 647.7 789l45.3-45.3L557.3 608zM704 192h160v736H160V192h160v64h384zm-320 0V96h256v96z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
