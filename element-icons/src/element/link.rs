// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn LinkIcon(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "m715.6 625.2-45.2-45.3 90.5-90.6c75-74.9 85.1-186.3 22.7-248.9-62.6-62.4-174-52.3-249 22.7l-90.4 90.5-45.3-45.2 90.5-90.5c100-100 252-110.1 339.5-22.7 87.5 87.5 77.3 239.4-22.7 339.5l-90.5 90.5zm-90.4 90.4-90.5 90.5c-100 100-252 110.1-339.5 22.7-87.5-87.5-77.3-239.4 22.7-339.5l90.5-90.5 45.2 45.3-90.5 90.6c-75 74.9-85.1 186.3-22.7 248.9 62.6 62.4 174 52.3 249-22.7l90.5-90.5zm0-362 45.2 45.2-271.6 271.6-45.2-45.2z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
