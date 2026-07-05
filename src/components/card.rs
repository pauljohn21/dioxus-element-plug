use dioxus::prelude::*;

// Card CSS class constants
pub const CARD: &str = "el-card";
pub const CARD_HEADER: &str = "el-card__header";
pub const CARD_BODY: &str = "el-card__body";
pub const CARD_SHADOW_ALWAYS: &str = "el-card--always";
pub const CARD_SHADOW_HOVER: &str = "el-card--hover";

/// Card props for the theme-chalk styled card component
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Card content
    pub children: Element,

    /// Card header content
    #[props(default)]
    pub header: Option<String>,

    /// Card shadow behavior
    #[props(default = "hover".to_string())]
    pub shadow: String,

    /// Card body style (inline CSS)
    #[props(default)]
    pub body_style: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles for the card
    #[props(default)]
    pub style: Option<String>,
}

/// A card component for organizing content
///
/// This component provides a container with optional header and customizable
/// shadow behavior for organizing related content.
///
/// ## Example
///
/// ```rust,ignore
/// use dioxus_element_plug::components::card::Card;
///
/// rsx! {
///     Card {
///         header: Some("Card Title".to_string()),
///         shadow: "always".to_string(),
///         
///         div {
///             "Card content goes here"
///         }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let mut class_names = vec!["el-card".to_string()];
    
    class_names.push(format!("is-{}-shadow", props.shadow));
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    let class_string = class_names.join(" ");
    let card_style = props.style.unwrap_or_default();
    let body_style = props.body_style.unwrap_or_default();
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{card_style}",
            
            if let Some(ref header_text) = props.header {
                div {
                    class: "el-card__header",
                    "{header_text}"
                }
            }
            
            div {
                class: "el-card__body",
                style: "{body_style}",
                {props.children}
            }
        }
    }
}

/// Panel component for grouping controls
#[derive(Props, Clone, PartialEq)]
pub struct PanelProps {
    /// Panel content
    pub children: Element,

    /// Panel title
    #[props(default)]
    pub title: Option<String>,

    /// Panel subtitle
    #[props(default)]
    pub subtitle: Option<String>,

    /// Panel type
    #[props(default = "default".to_string())]
    pub panel_type: String,

    /// Whether the panel is collapsible
    #[props(default = false)]
    pub collapsible: bool,

    /// Whether the panel is collapsed
    #[props(default = false)]
    pub collapsed: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,

    /// Toggle event handler
    #[props(default)]
    pub on_toggle: Option<EventHandler<MouseEvent>>,
}

/// A panel component for organizing form controls
///
/// This component provides a collapsible container for organizing
/// related form controls or content sections.
#[component]
pub fn Panel(props: PanelProps) -> Element {
    let mut class_names = vec!["el-panel".to_string()];
    
    class_names.push(format!("el-panel--{}", props.panel_type));
    
    if props.collapsible {
        class_names.push("is-collapsible".to_string());
    }
    
    if props.collapsed {
        class_names.push("is-collapsed".to_string());
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
            
            if props.collapsible {
                div {
                    class: "el-panel__header is-clickable",
                    onclick: move |event| {
                        if let Some(handler) = props.on_toggle {
                            handler.call(event);
                        }
                    },
                    
                    if let Some(ref title_text) = props.title {
                        h3 {
                            class: "el-panel__title",
                            "{title_text}"
                        }
                    }
                    
                    if let Some(ref sub_text) = props.subtitle {
                        span {
                            class: "el-panel__subtitle",
                            "{sub_text}"
                        }
                    }
                    
                    i {
                        class: if props.collapsed { "el-icon-arrow-down" } else { "el-icon-arrow-up" }
                    }
                }
            } else if let Some(ref title_text) = props.title {
                div {
                    class: "el-panel__header",
                    
                    h3 {
                        class: "el-panel__title",
                        "{title_text}"
                    }
                    
                    if let Some(ref sub_text) = props.subtitle {
                        span {
                            class: "el-panel__subtitle",
                            "{sub_text}"
                        }
                    }
                }
            }
            
            if !props.collapsible || !props.collapsed {
                div {
                    class: "el-panel__body",
                    {props.children}
                }
            }
        }
    }
}

/// Box component for flexible layouts
#[derive(Props, Clone, PartialEq)]
pub struct BoxProps {
    /// Box content
    pub children: Element,

    /// Box padding
    #[props(default)]
    pub padding: Option<String>,

    /// Box margin
    #[props(default)]
    pub margin: Option<String>,

    /// Box border radius
    #[props(default)]
    pub border_radius: Option<String>,

    /// Box background color
    #[props(default)]
    pub background: Option<String>,

    /// Box border
    #[props(default)]
    pub border: Option<String>,

    /// Box elevation/shadow
    #[props(default)]
    pub elevation: Option<u32>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A flexible box component for layouts
///
/// This component provides a customizable container with support for
/// padding, margin, borders, shadows, and background styling.
#[component]
pub fn Box(props: BoxProps) -> Element {
    let mut styles = vec![props.style.unwrap_or_default()];
    
    if let Some(padding) = props.padding {
        styles.push(format!("padding: {};", padding));
    }
    
    if let Some(margin) = props.margin {
        styles.push(format!("margin: {};", margin));
    }
    
    if let Some(border_radius) = props.border_radius {
        styles.push(format!("border-radius: {};", border_radius));
    }
    
    if let Some(background) = props.background {
        styles.push(format!("background: {};", background));
    }
    
    if let Some(border) = props.border {
        styles.push(format!("border: {};", border));
    }
    
    if let Some(elevation) = props.elevation {
        let shadow = match elevation {
            0 => "none",
            1 => "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)",
            2 => "0 3px 6px rgba(0,0,0,0.15), 0 2px 4px rgba(0,0,0,0.12)",
            3 => "0 10px 20px rgba(0,0,0,0.15), 0 3px 6px rgba(0,0,0,0.10)",
            4 => "0 15px 25px rgba(0,0,0,0.15), 0 5px 10px rgba(0,0,0,0.05)",
            5 => "0 20px 40px rgba(0,0,0,0.20)",
            _ => "0 25px 50px rgba(0,0,0,0.25)",
        };
        styles.push(format!("box-shadow: {};", shadow));
    }
    
    let style_string = styles.join("");
    
    let mut class_names = vec!["el-box".to_string()];
    
    if let Some(ref custom_class) = props.class {
        class_names.push(custom_class.to_string());
    }
    
    if let Some(elevation) = props.elevation {
        class_names.push(format!("el-box--elevation-{}", elevation));
    }
    
    let class_string = class_names.join(" ");
    
    rsx! {
        div {
            class: "{class_string}",
            style: "{style_string}",
            {props.children}
        }
    }
}

/// Accordion item for collapsible content
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Item title
    pub title: String,
    /// Item content
    pub content: String,
    /// Whether the item is disabled
    pub disabled: bool,
}

/// Accordion component for collapsible sections
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Accordion items
    pub items: Vec<AccordionItem>,

    /// Active item index (for controlled mode)
    #[props(default)]
    pub active_index: Option<usize>,

    /// Whether multiple items can be open simultaneously
    #[props(default = false)]
    pub accordion: bool,

    /// Whether to show animation
    #[props(default = true)]
    pub animated: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Inline styles
    #[props(default)]
    pub style: Option<String>,

    /// Change event handler
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
}

/// An accordion component for organizing collapsible content
///
/// This component provides a way to organize content in collapsible sections
/// that can be expanded or collapsed by the user.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut class_names = vec!["el-accordion".to_string()];
    
    if props.accordion {
        class_names.push("el-accordion--multiple".to_string());
    }
    
    if props.animated {
        class_names.push("el-accordion--animated".to_string());
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
            
            for (index, item) in props.items.iter().enumerate() {
                div {
                    class: "el-accordion__item",
                    
                    if !props.accordion || props.active_index == Some(index) {
                        div {
                            class: "el-accordion__header",
                            
                            button {
                                class: "el-accordion__button",
                                r#type: "button",
                                disabled: item.disabled,
                                onclick: move |_| {
                                    if let Some(handler) = props.on_change {
                                        handler.call(index);
                                    }
                                },
                                
                                span {
                                    class: "el-accordion__title",
                                    "{item.title}"
                                }
                                
                                i {
                                    class: "el-icon-arrow-down el-accordion__icon"
                                }
                            }
                        }
                        
                        div {
                            class: "el-accordion__content",
                            
                            div {
                                class: "el-accordion__body",
                                "{item.content}"
                            }
                        }
                    } else {
                        div {
                            class: "el-accordion__header",
                            
                            button {
                                class: "el-accordion__button",
                                r#type: "button",
                                disabled: item.disabled,
                                onclick: move |_| {
                                    if let Some(handler) = props.on_change {
                                        handler.call(index);
                                    }
                                },
                                
                                span {
                                    class: "el-accordion__title",
                                    "{item.title}"
                                }
                                
                                i {
                                    class: "el-icon-arrow-right el-accordion__icon"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}