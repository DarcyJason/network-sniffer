use dioxus::prelude::*;

// Import components from the components module
use crate::components::{
    Home, Docs, Settings, Navbar, PageNotFound
};

// Define the components module
mod components;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/docs")]
        Docs {},
        #[route("/settings")]
        Settings {},
    #[end_layout]
    #[route("/..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
