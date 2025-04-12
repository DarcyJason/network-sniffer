use dioxus::prelude::*;

#[component]
pub fn Docs() -> Element {
    rsx! {
        div {
            h1 { "Documentation" }
            p { "This is the documentation page." }
        }
    }
}
