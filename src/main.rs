use dioxus::prelude::*;

use crate::components::{Docs, Home, Navbar, Settings};

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
