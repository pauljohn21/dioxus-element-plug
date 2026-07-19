// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn Operation(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M389.4 768a96 96 0 0 1 181.2 0H896v64H570.6a96 96 0 0 1-181.2 0H128v-64zm192-288a96 96 0 0 1 181.2 0H896v64H762.6a96 96 0 0 1-181.2 0H128v-64zm-320-288a96 96 0 0 1 181.2 0H896v64H442.6a96 96 0 0 1-181.2 0H128v-64z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
