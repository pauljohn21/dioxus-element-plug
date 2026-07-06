use dioxus::prelude::*;

/// Container direction
#[derive(Clone, PartialEq)]
pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

/// Container props - 布局容器
#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    /// Container content
    pub children: Element,

    /// Direction
    #[props(default)]
    pub direction: Option<ContainerDirection>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Container component - layout container
///
/// Mirrors Element Plus `el-container`.
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut class_names = vec!["el-container".to_string()];

    if let Some(ref d) = props.direction {
        match d {
            ContainerDirection::Horizontal => class_names.push("is-horizontal".to_string()),
            ContainerDirection::Vertical => {}
        }
    }

    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");
    let style_string = props.style.unwrap_or_default();

    rsx! {
        section {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Header props - 顶栏容器
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// Header content
    pub children: Element,

    /// Height
    #[props(default = "60px".to_string())]
    pub height: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Header component - top bar container
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut class_names = vec!["el-header".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let style_string = format!("height: {};{}", props.height, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        header {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Footer props - 底栏容器
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// Footer content
    pub children: Element,

    /// Height
    #[props(default = "60px".to_string())]
    pub height: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Footer component - bottom bar container
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let mut class_names = vec!["el-footer".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let style_string = format!("height: {};{}", props.height, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        footer {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Main props - 主区域容器
#[derive(Props, Clone, PartialEq)]
pub struct MainProps {
    /// Main content
    pub children: Element,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Main component - main area container
#[component]
pub fn Main(props: MainProps) -> Element {
    let mut class_names = vec!["el-main".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
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

/// Aside props - 侧边栏容器
#[derive(Props, Clone, PartialEq)]
pub struct AsideProps {
    /// Aside content
    pub children: Element,

    /// Width
    #[props(default = "300px".to_string())]
    pub width: String,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Aside component - sidebar container
#[component]
pub fn Aside(props: AsideProps) -> Element {
    let mut class_names = vec!["el-aside".to_string()];
    if let Some(ref c) = props.class {
        class_names.push(c.clone());
    }
    let class_string = class_names.join(" ");

    let style_string = format!("width: {};{}", props.width, props.style.as_ref().map(|s| s.as_str()).unwrap_or(""));

    rsx! {
        aside {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
