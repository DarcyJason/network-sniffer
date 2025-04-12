use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            h1 { "Settings" }
            p { "This is the settings page." }
        }
    }
}
