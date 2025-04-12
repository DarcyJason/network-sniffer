mod capture;

use self::capture::Capture;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Welcome to Network-Sniffer"}
        Capture {  }
    }
}
