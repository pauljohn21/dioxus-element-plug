use dioxus::prelude::*;

/// Row justify type
#[derive(Clone, PartialEq)]
pub enum RowJustify {
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

impl RowJustify {
    pub fn as_class(&self) -> &'static str {
        match self {
            RowJustify::Start => "is-justify-start",
            RowJustify::End => "is-justify-end",
            RowJustify::Center => "is-justify-center",
            RowJustify::SpaceAround => "is-justify-space-around",
            RowJustify::SpaceBetween => "is-justify-space-between",
            RowJustify::SpaceEvenly => "is-justify-space-evenly",
        }
    }
}

/// Row align type
#[derive(Clone, PartialEq)]
pub enum RowAlign {
    Top,
    Middle,
    Bottom,
}

impl RowAlign {
    pub fn as_class(&self) -> &'static str {
        match self {
            RowAlign::Top => "is-align-top",
            RowAlign::Middle => "is-align-middle",
            RowAlign::Bottom => "is-align-bottom",
        }
    }
}

/// Row props - 栅格行
#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    /// Row content (Col components)
    pub children: Element,

    /// Grid spacing (px)
    #[props(default = 0)]
    pub gutter: u32,

    /// Flex layout justify
    #[props(default)]
    pub justify: Option<RowJustify>,

    /// Flex layout align
    #[props(default)]
    pub align: Option<RowAlign>,

    /// Whether flex layout
    #[props(default = false)]
    pub r#type: bool,

    /// Tag name
    #[props(default = "div".to_string())]
    pub tag: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Row component - grid row
///
/// Mirrors Element Plus `el-row`.
#[component]
pub fn Row(props: RowProps) -> Element {
    let mut class_names = vec!["el-row".to_string()];

    if props.r#type {
        class_names.push("el-row--flex".to_string());
    }

    if let Some(ref j) = props.justify {
        class_names.push(j.as_class().to_string());
    }
    if let Some(ref a) = props.align {
        class_names.push(a.as_class().to_string());
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let gutter_style = if props.gutter > 0 {
        format!("margin-left: -{}px; margin-right: -{}px;", props.gutter / 2, props.gutter / 2)
    } else {
        String::new()
    };
    let style_string = format!("{}{}", gutter_style, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Col props - 栅格列
#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    /// Col content
    pub children: Element,

    /// Number of columns spanned (1-24)
    #[props(default = 24)]
    pub span: u32,

    /// Number of columns offset
    #[props(default = 0)]
    pub offset: u32,

    /// Number of columns pushed
    #[props(default = 0)]
    pub push: u32,

    /// Number of columns pulled
    #[props(default = 0)]
    pub pull: u32,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Col component - grid column
///
/// Mirrors Element Plus `el-col`.
#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class_names = vec![format!("el-col-{}", props.span)];

    if props.offset > 0 {
        class_names.push(format!("el-col-offset-{}", props.offset));
    }
    if props.push > 0 {
        class_names.push(format!("el-col-push-{}", props.push));
    }
    if props.pull > 0 {
        class_names.push(format!("el-col-pull-{}", props.pull));
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
