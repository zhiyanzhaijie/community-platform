use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route_str = route.join("/");
    rsx! {
        div { "404" }
    }
}
