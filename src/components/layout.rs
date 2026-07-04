use dioxus::prelude::*;
use crate::theme::classes;

/// Container props
#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub direction: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Container component for layout
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut class_names = vec![classes::CONTAINER];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
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

/// Header props
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    #[props(default)]
    pub children: Element,

    #[props(default = 60)]
    pub height: u32,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Header component
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut class_names = vec![classes::HEADER];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
    }

    let class_string = class_names.join(" ");
    let style_string = format!(
        "height: {}px;{}",
        props.height,
        props.style.unwrap_or_default()
    );

    rsx! {
        header {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Aside props
#[derive(Props, Clone, PartialEq)]
pub struct AsideProps {
    #[props(default)]
    pub children: Element,

    #[props(default = 200)]
    pub width: u32,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Aside component
#[component]
pub fn Aside(props: AsideProps) -> Element {
    let mut class_names = vec![classes::ASIDE];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
    }

    let class_string = class_names.join(" ");
    let style_string = format!(
        "width: {}px;{}",
        props.width,
        props.style.unwrap_or_default()
    );

    rsx! {
        aside {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Main props
#[derive(Props, Clone, PartialEq)]
pub struct MainProps {
    #[props(!optional)]
    pub children: Element,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Main content component
#[component]
pub fn Main(props: MainProps) -> Element {
    let mut class_names = vec![classes::MAIN];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
    }

    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        main {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Footer props
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    #[props(default)]
    pub children: Element,

    #[props(default = 60)]
    pub height: u32,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Footer component
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let mut class_names = vec![classes::FOOTER];

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class);
    }

    let class_string = class_names.join(" ");
    let style_string = format!(
        "height: {}px;{}",
        props.height,
        props.style.unwrap_or_default()
    );

    rsx! {
        footer {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Row props for grid system
#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    #[props(!optional)]
    pub children: Element,

    #[props(default)]
    pub gutter: Option<u32>,

    #[props(default)]
    pub justify: Option<String>,

    #[props(default)]
    pub align: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Row component for grid layout
#[component]
pub fn Row(props: RowProps) -> Element {
    let mut class_names = vec![classes::ROW.to_string()];

    if let Some(ref justify) = props.justify {
        class_names.push(format!("is-justify-{}", justify));
    }

    if let Some(ref align) = props.align {
        class_names.push(format!("is-align-{}", align));
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }

    let class_string = class_names.join(" ");

    let mut style_parts = vec![props.style.unwrap_or_default()];

    if let Some(gutter) = props.gutter {
        let gutter_margin = -(gutter as i32 / 2);
        style_parts.push(format!("margin-left: {}px; margin-right: {}px;", gutter_margin, gutter_margin));
    }

    let style_string = style_parts.join("");

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Col props for grid system
#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    #[props(!optional)]
    pub children: Element,

    #[props(default)]
    pub span: Option<u32>,

    #[props(default)]
    pub offset: Option<u32>,

    #[props(default)]
    pub push: Option<u32>,

    #[props(default)]
    pub pull: Option<u32>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,
}

/// Column component for grid layout
#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class_names = vec![classes::COL.to_string()];

    if let Some(span) = props.span {
        class_names.push(format!("el-col-{}", span));
    }

    if let Some(offset) = props.offset {
        class_names.push(format!("el-col-offset-{}", offset));
    }

    if let Some(push) = props.push {
        class_names.push(format!("el-col-push-{}", push));
    }

    if let Some(pull) = props.pull {
        class_names.push(format!("el-col-pull-{}", pull));
    }

    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
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
