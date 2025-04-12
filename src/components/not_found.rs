use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}