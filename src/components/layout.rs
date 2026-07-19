use dioxus::prelude::*;

use crate::components::common::{ClassBuilder, style_str};

// Layout CSS class constants
pub const CONTAINER: &str = "el-container";
pub const HEADER: &str = "el-header";
pub const ASIDE: &str = "el-aside";
pub const MAIN: &str = "el-main";
pub const FOOTER: &str = "el-footer";
pub const ROW: &str = "el-row";
pub const COL: &str = "el-col";

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
    let class_string = ClassBuilder::new(CONTAINER)
        .add_opt_str(props.direction.as_deref().map(|d| format!("el-container--{}", d)).as_deref())
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

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
    let class_string = ClassBuilder::new(HEADER)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = format!(
        "height: {}px;{}",
        props.height,
        style_str(&props.style)
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
    let class_string = ClassBuilder::new(ASIDE)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = format!(
        "width: {}px;{}",
        props.width,
        style_str(&props.style)
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
    let class_string = ClassBuilder::new(MAIN)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

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
    let class_string = ClassBuilder::new(FOOTER)
        .add_opt(props.class.as_ref())
        .build();
    let style_string = format!(
        "height: {}px;{}",
        props.height,
        style_str(&props.style)
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
    let class_string = ClassBuilder::new(ROW)
        .add_opt_str(props.justify.as_ref().map(|j| format!("is-justify-{}", j)).as_deref())
        .add_opt_str(props.align.as_ref().map(|a| format!("is-align-{}", a)).as_deref())
        .add_opt(props.class.as_ref())
        .build();

    let mut style_parts = vec![style_str(&props.style)];

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
    let class_string = ClassBuilder::new(COL)
        .add_opt_str(props.span.as_ref().map(|s| format!("el-col-{}", s)).as_deref())
        .add_opt_str(props.offset.as_ref().map(|o| format!("el-col-offset-{}", o)).as_deref())
        .add_opt_str(props.push.as_ref().map(|p| format!("el-col-push-{}", p)).as_deref())
        .add_opt_str(props.pull.as_ref().map(|p| format!("el-col-pull-{}", p)).as_deref())
        .add_opt(props.class.as_ref())
        .build();
    let style_string = style_str(&props.style);

    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
