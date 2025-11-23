use crate::views::{Home, Login, NotFound, Signup, ToolDetail, ToolList};
use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};

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
    let nav = use_navigator();

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            // Navigation bar
            nav {
                class: "bg-white shadow-sm sticky top-0 z-50 border-b border-gray-200",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex justify-between h-16",
                        div {
                            class: "flex items-center space-x-8",
                            h1 {
                                class: "text-2xl font-bold text-gray-900 cursor-pointer",
                                onclick: move |_| { nav.push(Route::Home {}); },
                                "Dioxus Router Demo"
                            }
                            div {
                                class: "flex items-center space-x-2",
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    on_click: move |_| { nav.push(Route::Home {}); },
                                    "Home"
                                }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    on_click: move |_| { nav.push(Route::ToolList {}); },
                                    "Tools"
                                }
                            }
                        }
                        div {
                            class: "flex items-center space-x-4",
                            Button {
                                variant: ButtonVariant::Ghost,
                                on_click: move |_| { nav.push(Route::Login {}); },
                                "Log in"
                            }
                            Button {
                                on_click: move |_| { nav.push(Route::Signup {}); },
                                "Sign up"
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
