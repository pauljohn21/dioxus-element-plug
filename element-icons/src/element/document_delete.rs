// @generated from Element Plus SVG. Do not edit.

use dioxus::core::{Element, Template, TemplateNode};

use crate::IconProps;
use crate::vdom::{attr, icon_element, icon_template, path, svg};

#[allow(non_snake_case)]
pub fn DocumentDelete(props: IconProps) -> Element {
    static TEMPLATE_ROOTS: &[TemplateNode] = &[svg(&[
        path(&[attr("fill", "currentColor"), attr("d", "M805.5 320 640 154.5V320zm26.5 64H576V128H192v768h640zM160 64h480l256 256v608a32 32 0 0 1-32 32H160a32 32 0 0 1-32-32V96a32 32 0 0 1 32-32m309 546.3-90.5-90.6 45.2-45.3 90.6 90.5 90.5-90.4 45.2 45.2-90.4 90.6 90.4 90.5-45.2 45.2-90.5-90.4-90.6 90.4-45.2-45.2z")]),
    ])];
    static TEMPLATE: Template = icon_template(TEMPLATE_ROOTS);

    icon_element(TEMPLATE, "0 0 1024 1024", props)
}
