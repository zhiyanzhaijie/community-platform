use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::button::Button;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let _route_str = route.join("/");
    let nav = use_navigator();

    rsx! {
        div {
            class: "flex flex-col items-center justify-center py-20 text-center",
            h1 {
                class: "text-6xl font-bold text-gray-900 mb-4",
                "404"
            }
            p {
                class: "text-xl text-gray-600 mb-8",
                "Page not found"
            }
            Button {
                on_click: move |_| { nav.push(Route::Home {}); },
                "Go Home"
            }
        }
    }
}
