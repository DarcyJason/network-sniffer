use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link {
                        to: "/",
                        "Home"
                    }
                }
                li {
                    Link {
                        to: "/docs",
                        "Docs"
                    }
                }
                li {
                    Link {
                        to: "/settings",
                        "Settings"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}