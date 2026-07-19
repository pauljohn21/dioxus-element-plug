use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

mod components;
mod pages;

use components::layout::DocLayout;
use pages::*;

fn main() {
    // Set up panic hook for better WASM error messages
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    dioxus::launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    // All routes inside DocLayout will have Header + Sidebar + Outlet
    #[layout(DocLayout)]
    #[route("/")]
    Index {},

    #[route("/guide/installation")]
    Installation {},
    #[route("/guide/quickstart")]
    QuickStart {},

    #[route("/component/:name")]
    ComponentPage { name: String },
}

#[component]
fn App() -> Element {
    // Create dark mode signal and provide it to all descendants via context
    let is_dark = use_signal(|| false);
    use_context_provider(|| is_dark);

    let theme = if is_dark() {
        Theme::dark()
    } else {
        Theme::light()
    };

    let styles = CompleteStyleManager::new()
        .with_theme(theme)
        .generate_complete_styles();

    rsx! {
        style { "{styles}" }
        Router::<Route> {}
    }
}
