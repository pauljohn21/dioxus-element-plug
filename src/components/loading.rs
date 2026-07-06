use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LoadingProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let class_string = format!("el-loading{}", props.class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default());
    let style_string = props.style.clone().unwrap_or_default();
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
