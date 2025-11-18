use dioxus::prelude::*;
mod views;
mod components;

use components::{
    button::{Button, ButtonVariant},
    separator::Separator,
    tabs::{Tabs, TabList, TabTrigger, TabContent},
    accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent},
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENTS_THEME: Asset = asset!("/assets/dx-components-theme.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[nest("/tool")]
    #[route("/")]
    ToolList {},
    #[route("/:id")]
    ToolDetail { id: String },
    #[end_nest]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND }
        document::Stylesheet { href: DX_COMPONENTS_THEME }
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
                                Separator { horizontal: false, class: "h-6" }
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

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-3xl font-bold text-gray-900",
                "Welcome to Dioxus Router Demo"
            }
            p {
                class: "text-lg text-gray-600 mb-6",
                "This demo showcases Dioxus components and routing capabilities."
            }

            Tabs {
                default_value: "features",
                class: "w-full",
                TabList {
                    class: "flex border-b border-gray-200 mb-4",
                    TabTrigger {
                        value: "features",
                        index: 0usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Features"
                    }
                    TabTrigger {
                        value: "routing",
                        index: 1usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Routing"
                    }
                    TabTrigger {
                        value: "components",
                        index: 2usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Components"
                    }
                }

                TabContent {
                    value: "features",
                    index: 0usize,
                    class: "space-y-4",
                    h3 {
                        class: "text-xl font-semibold text-gray-900",
                        "Key Features"
                    }
                    ul {
                        class: "list-disc list-inside space-y-2 text-gray-600",
                        li { "Cross-platform UI framework built in Rust" }
                        li { "Reactive state management with signals" }
                        li { "Component-based architecture" }
                        li { "Type-safe routing" }
                    }
                }

                TabContent {
                    value: "routing",
                    index: 1usize,
                    class: "space-y-4",
                    h3 {
                        class: "text-xl font-semibold text-gray-900 mb-4",
                        "Try these routes:"
                    }
                    ul {
                        class: "space-y-3",
                        li {
                            Link {
                                to: Route::ToolList {},
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full justify-start",
                                    "/tool - Tool List"
                                }
                            }
                        }
                        li {
                            Link {
                                to: Route::ToolDetail { id: "typescript".to_string() },
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full justify-start",
                                    "/tool/typescript - Tool Detail"
                                }
                            }
                        }
                        li {
                            Link {
                                to: Route::ToolDetail { id: "rust".to_string() },
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full justify-start",
                                    "/tool/rust - Tool Detail"
                                }
                            }
                        }
                    }
                }

                TabContent {
                    value: "components",
                    index: 2usize,
                    class: "space-y-4",
                    h3 {
                        class: "text-xl font-semibold text-gray-900",
                        "Dioxus Components Used"
                    }
                    ul {
                        class: "list-disc list-inside space-y-2 text-gray-600",
                        li { "Button - Interactive buttons with variants" }
                        li { "Separator - Visual dividers" }
                        li { "Tabs - Tabbed interface (you're using it now!)" }
                        li { "Accordion - Collapsible content sections" }
                    }
                }
            }
        }
    }
}

#[component]
fn ToolList() -> Element {
    let tools = vec![
        ("typescript", "TypeScript", "Typed superset of JavaScript", "A typed superset of JavaScript that compiles to clean, readable JavaScript code. It adds static typing and other features that help catch errors during development."),
        ("rust", "Rust", "Systems programming language", "A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Perfect for performance-critical applications."),
        ("dioxus", "Dioxus", "Rust UI framework", "A reactive UI framework for Rust. Dioxus uses a virtual DOM and declarative syntax similar to React, making it perfect for building web, desktop, and mobile apps."),
    ];

    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-3xl font-bold text-gray-900",
                "Tools"
            }
            p {
                class: "text-gray-600 mb-6",
                "Browse our collection of tools using the accordion below:"
            }

            Accordion {
                class: "space-y-4",
                collapsible: true,
                for (index, (id, name, short_desc, long_desc)) in tools.iter().enumerate() {
                    AccordionItem {
                        index,
                        class: "bg-white border border-gray-200 rounded-lg overflow-hidden",
                        AccordionTrigger {
                            class: "w-full px-6 py-4 text-left hover:bg-gray-50 transition-colors",
                            div {
                                class: "flex items-center justify-between",
                                div {
                                    h3 {
                                        class: "text-xl font-semibold text-gray-900",
                                        "{name}"
                                    }
                                    p {
                                        class: "text-sm text-gray-600 mt-1",
                                        "{short_desc}"
                                    }
                                }
                            }
                        }
                        AccordionContent {
                            class: "px-6 py-4 border-t border-gray-200 bg-gray-50",
                            p {
                                class: "text-gray-700 mb-4",
                                "{long_desc}"
                            }
                            Link {
                                to: Route::ToolDetail { id: id.to_string() },
                                Button {
                                    variant: ButtonVariant::Primary,
                                    class: "mt-2",
                                    "View Details →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ToolDetail(id: String) -> Element {
    let details = match id.as_str() {
        "typescript" => ("TypeScript", "A typed superset of JavaScript that compiles to clean, readable JavaScript code. It adds static typing and other features that help catch errors during development."),
        "rust" => ("Rust", "A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Perfect for performance-critical applications."),
        "dioxus" => ("Dioxus", "A reactive UI framework for Rust. Dioxus uses a virtual DOM and declarative syntax similar to React, making it perfect for building web, desktop, and mobile apps."),
        _ => ("Unknown Tool", "This tool doesn't exist in our database."),
    };

    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-3xl font-bold text-gray-900",
                "{details.0}"
            }
            p {
                class: "text-lg text-gray-600 leading-relaxed",
                "{details.1}"
            }
            div {
                class: "mt-8 pt-8 border-t border-gray-200",
                Link {
                    to: Route::ToolList {},
                    Button {
                        variant: ButtonVariant::Outline,
                        "← Back to Tools"
                    }
                }
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
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
