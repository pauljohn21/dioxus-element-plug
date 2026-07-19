// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Cloudy(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M598.4 831.9H328.2a256 256 0 0 1-34.5-510.6A352 352 0 1 1 598.4 832m-271.4-64h272.3a288 288 0 1 0-248.5-417.7L335 381.4l-34.8 3.6A192 192 0 0 0 327.1 768z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
