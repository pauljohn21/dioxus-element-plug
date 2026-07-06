use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let class_string = format!("el-time-picker{}", props.class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default());
    let style_string = props.style.clone().unwrap_or_default();
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}
