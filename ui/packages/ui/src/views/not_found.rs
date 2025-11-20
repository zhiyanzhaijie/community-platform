use dioxus::prelude::*;
use crate::components::button::{Button, ButtonVariant};
use crate::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route_str = route.join("/");
    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-3xl font-bold text-gray-900",
                "404 - Page Not Found"
            }
            p {
                class: "text-lg text-gray-600",
                "Sorry, the page you're looking for doesn't exist."
            }
            p {
                class: "text-gray-500 font-mono text-sm",
                "Route: /{route_str}"
            }
            Link {
                to: Route::Home {},
                Button {
                    variant: ButtonVariant::Primary,
                    class: "mt-4",
                    "Go Home"
                }
            }
        }
    }
}
