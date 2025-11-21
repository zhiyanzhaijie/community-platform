use crate::views::{Home, Login, NotFound, Signup, ToolDetail, ToolList};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[nest("/tool")]
    #[route("/")]
    ToolList {},
    #[route("/:id")]
    ToolDetail { id: String },
    #[end_nest]
    #[nest("/auth")]
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[end_nest]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            // Navigation bar
            nav {
                class: "bg-white shadow-sm",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex justify-between h-16",
                        div {
                            class: "flex items-center space-x-8",
                            h1 {
                                class: "text-2xl font-bold text-gray-900",
                                "Dioxus Router Demo"
                            }
                            div {
                                class: "flex items-center space-x-2",
                                Link {
                                    to: Route::Home {},
                                    class: "text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium",
                                    "Home"
                                }
                                Link {
                                    to: Route::ToolList {},
                                    class: "text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium",
                                    "Tools"
                                }
                            }
                        }
                    }
                }
            }
            // Page content
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                Outlet::<Route> {}
            }
        }
    }
}
