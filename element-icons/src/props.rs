use dioxus::core::{AttributeValue, IntoAttributeValue};
use dioxus::prelude::*;

/// 图标尺寸便捷类型
#[derive(Clone, PartialEq)]
pub struct IconSize(AttributeValue);

impl IconSize {
    pub(crate) fn into_value(self) -> AttributeValue {
        self.0
    }
}

impl Default for IconSize {
    fn default() -> Self {
        Self(AttributeValue::Int(24))
    }
}

impl<T> From<T> for IconSize
where
    T: IntoAttributeValue,
{
    fn from(value: T) -> Self {
        Self(value.into_value())
    }
}

/// 所有 Element Plus 图标组件共享的属性
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// 便捷尺寸，用于设置 SVG 的 width 和 height（当未直接设置这些属性时）
    #[props(into, default = IconSize::default())]
    pub size: IconSize,
    /// 传递给根 SVG 元素的属性
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
}
