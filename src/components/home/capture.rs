use dioxus::prelude::*;

#[component]
pub fn Capture() -> Element {
    rsx! {
        div {
            input { type: "text", placeholder: "Enter a capture filter ..." }
        }
    }
}
